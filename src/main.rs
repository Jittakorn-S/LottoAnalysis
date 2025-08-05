use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, error};

// --- Project Modules ---
mod analysis;
mod models;
mod scraper;

// --- Imports from Modules ---
use models::{AnalyzeRequest, AppState, StartScrapeRequest};

// --- Performance Optimization ---
// Set mimalloc as the global memory allocator. This can improve performance
// by replacing the system's default allocator with a faster one.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// --- API Handlers ---

/// Serves the main index.html file.
async fn index() -> impl Responder {
    match std::fs::read_to_string("templates/index.html") {
        Ok(content) => HttpResponse::Ok().content_type("text/html; charset=utf-8").body(content),
        Err(_) => HttpResponse::InternalServerError().body("ไม่สามารถอ่านไฟล์ index.html ได้"),
    }
}

/// Starts the scraping process in a non-blocking background thread.
async fn start_scrape(
    req: web::Json<StartScrapeRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let mut status = app_state.task_status.lock().unwrap();
    if status.is_running {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "มีโปรแกรมดึงข้อมูลทำงานอยู่แล้ว"
        }));
    }

    status.is_running = true;
    status.lotto_type = Some(req.lotto_type.to_string());
    status.progress = vec![format!("🚀 กำลังเริ่มโปรแกรมดึงข้อมูลสำหรับสลาก {}...", req.lotto_type)];
    status.results.clear();

    let lotto_type = req.lotto_type.clone();
    let app_state_clone = app_state.clone();

    tokio::spawn(async move {
        scraper::run_scraper(lotto_type, app_state_clone).await;
    });

    HttpResponse::Accepted().json(serde_json::json!({
        "message": format!("เริ่มกระบวนการดึงข้อมูลสำหรับสลาก {} แล้ว!", req.lotto_type)
    }))
}

/// Returns the current status of the scraping task.
async fn get_status(app_state: web::Data<AppState>) -> impl Responder {
    let status = app_state.task_status.lock().unwrap();
    HttpResponse::Ok().json(&*status)
}

/// Handles analysis requests by delegating to the analysis module.
/// OPTIMIZATION: The analysis logic is wrapped in `web::block` to run it in a
/// separate thread pool. This prevents the CPU-intensive analysis from blocking
//  the main Actix-web server threads, ensuring the server remains responsive.
async fn analyze_handler(req: web::Json<AnalyzeRequest>) -> Result<HttpResponse, error::Error> {
    let result = web::block(move || analysis::run_analysis(&req))
        .await
        .map_err(error::ErrorInternalServerError)?; // Handle thread pool errors

    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() }))),
    }
}

// --- Server Setup ---
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_str = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().expect("PORT must be a valid number");
    let app_state = web::Data::new(AppState::new());

    println!("✅ เซิร์ฟเวอร์กำลังเริ่มทำงานที่ http://0.0.0.0:{}", port);
    println!("⚡️ ตัวจัดสรรหน่วยความจำ: mimalloc");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/start-scrape", web::post().to(start_scrape))
            .route("/status", web::get().to(get_status))
            .route("/analyze", web::post().to(analyze_handler))
            .service(Files::new("/static", "static"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}