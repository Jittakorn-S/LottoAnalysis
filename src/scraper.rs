use crate::models::{AppState, LottoResult, LottoType};
use actix_web::web;
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};
use anyhow::{Result, anyhow};


/// Main scraper function that builds a tolerant HTTP client and dispatches to the correct scraper.
pub async fn run_scraper(lotto_type: LottoType, app_state: web::Data<AppState>) {
    // Create a custom reqwest client that ignores SSL certificate errors.
    let client = match reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            let mut status = app_state.task_status.lock().unwrap();
            status.progress.push(format!("⚠️ ข้อผิดพลาดร้ายแรง: ไม่สามารถสร้าง HTTP client: {}", e));
            status.is_running = false;
            return;
        }
    };

    let start_url = match lotto_type {
        LottoType::Thai => "https://news.sanook.com/lotto/archive/".to_string(),
        LottoType::Laos => "https://expserve.com/backward/laosdevelops".to_string(),
    };

    let mut all_results = Vec::new();
    let mut current_url = Some(start_url);

    // Loop through all pages until there is no "Next Page" link.
    while let Some(url) = current_url {
        {
            let mut status = app_state.task_status.lock().unwrap();
            status.progress.push(format!("📄 กำลังดึงข้อมูลหน้า: {}", url));
        }

        // Dispatch to the correct scraper function based on the selected lotto type.
        let scrape_result = match lotto_type {
            LottoType::Thai => scrape_thai_page(&client, &url).await,
            LottoType::Laos => scrape_laos_page(&client, &url).await,
        };

        match scrape_result {
            Ok((mut page_results, next_url)) => {
                all_results.append(&mut page_results);
                current_url = next_url; // This will be `None` on the last page, stopping the loop.
            }
            Err(e) => {
                let mut status = app_state.task_status.lock().unwrap();
                status.progress.push(format!("⚠️ เกิดข้อผิดพลาดในการดึงข้อมูลหน้า {}: {}", url, e));
                current_url = None; // Stop on error.
            }
        }
        sleep(Duration::from_millis(500)).await; // Be polite to the server.
    }

    // Update the final status once scraping is complete.
    let mut status = app_state.task_status.lock().unwrap();
    status.results = all_results;
    status.progress.push(format!("✅ การดึงข้อมูลสลาก {}เสร็จสมบูรณ์", lotto_type));
    status.is_running = false;
}

/// Scrapes a single page of Thai lottery results from news.sanook.com.
async fn scrape_thai_page(
    client: &reqwest::Client,
    url: &str,
) -> Result<(Vec<LottoResult>, Option<String>)> {
    let resp_text = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&resp_text);

    let article_selector = Selector::parse(r#"article.archive--lotto"#).unwrap();
    let date_selector = Selector::parse(r#"time.archive--lotto__date"#).unwrap();
    let li_selector = Selector::parse(r#"ul.archive--lotto__result-list li"#).unwrap();
    let label_selector = Selector::parse(r#"em.archive--lotto__result-txt"#).unwrap();
    let number_selector = Selector::parse(r#"strong.archive--lotto__result-number"#).unwrap();
    let next_button_selector = Selector::parse(r#"a.pagination__item--next"#).unwrap();

    let mut page_results = Vec::new();
    for article in document.select(&article_selector) {
        let draw_date = article.select(&date_selector).next()
            .and_then(|time| time.value().attr("datetime")).unwrap_or("Unknown").to_string();

        let mut first_prize = None;
        let mut last_2_digits = None;

        for li in article.select(&li_selector) {
            let label = li.select(&label_selector).next().map(|em| em.text().collect::<String>());
            let prize = li.select(&number_selector).next().map(|s| s.text().collect::<String>());
            if let (Some(label_text), Some(prize_text)) = (label, prize) {
                if label_text.contains("รางวัลที่ 1") { first_prize = Some(prize_text.trim().to_string()); }
                else if label_text.contains(r#"เลขท้าย 2 ตัว"#) { last_2_digits = Some(prize_text.trim().to_string()); }
            }
        }

        if let (Some(fp), Some(l2d)) = (first_prize, last_2_digits) {
            page_results.push(LottoResult { draw_date, prize1: fp, prize2: l2d });
        }
    }
    let next_page_url = document.select(&next_button_selector).next()
        .and_then(|a| a.value().attr("href")).map(String::from);

    Ok((page_results, next_page_url))
}

/// Scrapes a single page of Laos lottery results from expserve.com.
async fn scrape_laos_page(
    client: &reqwest::Client,
    url: &str,
) -> Result<(Vec<LottoResult>, Option<String>)> {
    let resp_text = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&resp_text);

    let row_selector = Selector::parse(r#"div.m_410352e9.mantine-Grid-root"#).unwrap();

    let mut page_results = Vec::new();
    for row in document.select(&row_selector).skip(1) { // Skip header row
        let cols: Vec<String> = row.select(&Selector::parse("div.mantine-Grid-col").unwrap())
                                  .map(|div| div.text().collect::<String>())
                                  .collect();
        
        if cols.len() >= 3 {
            let date_text = cols[0].split('|').last().unwrap_or("").trim().to_string();
            let prize1 = cols[1].trim().to_string(); // 3-digit prize
            let prize2 = cols[2].trim().to_string(); // 2-digit prize

            if date_text.is_empty() || prize1.is_empty() || prize2.is_empty() || prize1 == "งดออกผล" {
                continue;
            }

            page_results.push(LottoResult { draw_date: date_text, prize1, prize2 });
        }
    }

    if page_results.is_empty() && document.select(&row_selector).count() > 1 {
         return Err(anyhow!("ไม่สามารถแยกวิเคราะห์ผลลัพธ์ใดๆ ได้ แม้ว่าจะพบแถวก็ตาม โครงสร้าง HTML อาจมีการเปลี่ยนแปลง"));
    }

    // --- Automatic Next Page Logic ---
    let mut next_page_url: Option<String> = None;
    let anchor_selector = Selector::parse("a").unwrap();
    let label_selector = Selector::parse(r#"span.mantine-Button-label"#).unwrap();

    for link_element in document.select(&anchor_selector) {
        if let Some(label_span) = link_element.select(&label_selector).next() {
            if label_span.text().collect::<String>().trim() == "หน้าต่อไป" {
                if let Some(href) = link_element.value().attr("href") {
                    let base_url = reqwest::Url::parse(url)?;
                    let next_url = base_url.join(href)?;
                    next_page_url = Some(next_url.to_string());
                    break;
                }
            }
        }
    }

    Ok((page_results, next_page_url))
}