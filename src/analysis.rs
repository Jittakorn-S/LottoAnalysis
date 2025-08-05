use crate::models::{AnalysisMethod, AnalyzeRequest, AnalysisResponse};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

// Machine Learning & Stats
use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array, Array1, Array2};

/// Main analysis router. It receives a request and calls the appropriate analysis function.
pub fn run_analysis(req: &AnalyzeRequest) -> Result<AnalysisResponse> {
    match req.method {
        AnalysisMethod::ComprehensiveStatistics => run_comprehensive_analysis(&req.numbers),
        AnalysisMethod::Numerology => run_numerology_analysis(&req.numbers),
        AnalysisMethod::MlDecisionTree => run_ml_analysis(&req.numbers),
        AnalysisMethod::MarkovChain => run_markov_chain_analysis(&req.numbers),
    }
}

// --- Method 1: Statistical Analysis (Optimized) ---
fn run_comprehensive_analysis(numbers_str: &[String]) -> Result<AnalysisResponse> {
    if numbers_str.len() < 10 {
        return Err(anyhow!("ข้อมูลไม่เพียงพอ AI ต้องการชุดตัวเลขอย่างน้อย 10 ชุด แต่พบเพียง {}.", numbers_str.len()));
    }

    // --- PERFORMANCE OPTIMIZATION ---
    // Calculate all statistics in a single pass to avoid multiple iterations.
    let mut sum = 0.0;
    let mut counts = HashMap::new();
    let mut numbers_f64 = Vec::with_capacity(numbers_str.len());

    for s in numbers_str {
        if let Ok(num) = s.parse::<f64>() {
            sum += num;
            numbers_f64.push(num);
        }
        *counts.entry(s.clone()).or_insert(0) += 1;
    }

    if numbers_f64.len() < 5 {
        return Err(anyhow!("ไม่สามารถแยกวิเคราะห์ตัวเลขที่ถูกต้องเพียงพอสำหรับการวิเคราะห์ทางสถิติ"));
    }
    
    // Sort once for Median, Min, and Max.
    numbers_f64.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let mean = sum / (numbers_f64.len() as f64);
    let median = numbers_f64[numbers_f64.len() / 2];
    
    let min = *numbers_f64.first().unwrap_or(&0.0);
    let max = *numbers_f64.last().unwrap_or(&0.0);
    
    // Calculate variance and standard deviation
    let variance = numbers_f64.iter().map(|&val| (val - mean).powi(2)).sum::<f64>() / (numbers_f64.len() as f64);
    let std_dev = variance.sqrt();
    
    let mode_str = counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let statistical_summary = HashMap::from([
        ("ขนาดชุดข้อมูล".to_string(), numbers_str.len().to_string()),
        ("ค่าเฉลี่ย".to_string(), format!("{:.2}", mean)),
        ("มัธยฐาน".to_string(), format!("{:.2}", median)),
        ("ฐานนิยม".to_string(), mode_str.clone()),
        ("ส่วนเบี่ยงเบนมาตรฐาน".to_string(), format!("{:.2}", std_dev)),
        ("พิสัย".to_string(), format!("{} - {}", min, max)),
    ]);

    let prediction_output = HashMap::from([
        ("PREDICTION".to_string(), serde_json::json!(mode_str.clone())),
        ("METHOD".to_string(), serde_json::json!("ฐานนิยมทางสถิติ")),
    ]);

    let detailed_explanation = HashMap::from([
        ("หลักการ".to_string(), "โมเดลนี้ใช้การวิเคราะห์ความถี่ (Frequency Analysis) โดยการทำนายคือ 'ฐานนิยม' (ตัวเลขที่เกิดขึ้นบ่อยที่สุด) เนื่องจากเป็นตัวบ่งชี้ทางสถิติที่แข็งแกร่งของการเกิดซ้ำ".to_string()),
        ("ตรรกะการทำนาย".to_string(), format!("ตัวเลข '{}' ปรากฏบ่อยที่สุดในข้อมูลย้อนหลังที่ให้มา", mode_str)),
    ]);

    Ok(AnalysisResponse {
        statistical_summary,
        pattern_analysis: HashMap::new(),
        prediction_output,
        detailed_explanation,
    })
}

// --- Method 2: Numerology (Digital Root) ---
fn calculate_digital_root(number_str: &str) -> u32 {
    let mut sum: u32 = number_str.chars().filter_map(|c| c.to_digit(10)).sum();
    while sum > 9 {
        sum = sum.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    }
    sum
}

fn run_numerology_analysis(numbers_str: &[String]) -> Result<AnalysisResponse> {
    if numbers_str.is_empty() { return Err(anyhow!("ไม่สามารถวิเคราะห์รายการตัวเลขที่ว่างเปล่าได้")); }

    let mut root_counts = HashMap::new();
    for num_str in numbers_str {
        *root_counts.entry(calculate_digital_root(num_str)).or_insert(0) += 1;
    }

    let most_common_root = root_counts.iter().max_by_key(|&(_, count)| count)
        .map(|(root, _)| *root).unwrap_or(0);

    let prediction = numbers_str.iter().rev()
        .find(|n| calculate_digital_root(n) == most_common_root)
        .cloned().unwrap_or_else(|| "N/A".to_string());

    Ok(AnalysisResponse {
        statistical_summary: HashMap::from([
            ("ประเภทการวิเคราะห์".to_string(), "ศาสตร์แห่งตัวเลข (Digital Root)".to_string()),
            ("รากที่พบบ่อยที่สุด".to_string(), most_common_root.to_string()),
        ]),
        pattern_analysis: HashMap::from([("ความถี่ของ Digital Root".to_string(), serde_json::json!(root_counts))]),
        prediction_output: HashMap::from([
            ("PREDICTION".to_string(), serde_json::json!(prediction)),
            ("METHOD".to_string(), serde_json::json!("การวิเคราะห์ Digital Root")),
        ]),
        detailed_explanation: HashMap::from([("หลักการ".to_string(), "การวิเคราะห์นี้จะคำนวณ 'digital root' (ผลรวมเลขหลักเดียว) ของแต่ละตัวเลข การทำนายคือตัวเลขล่าสุดในอดีตที่ตรงกับ digital root ที่พบบ่อยที่สุด".to_string())]),
    })
}

// --- Method 3: Machine Learning (Decision Tree) ---
fn run_ml_analysis(numbers_str: &[String]) -> Result<AnalysisResponse> {
    if numbers_str.len() < 10 {
        return Err(anyhow!("แมชชีนเลิร์นนิงต้องการข้อมูลอย่างน้อย 10 จุดข้อมูลในการฝึกฝน"));
    }
    let num_len = numbers_str.first().map_or(0, |n| n.len());
    if num_len == 0 || !numbers_str.iter().all(|n| n.len() == num_len && n.chars().all(|c| c.is_digit(10))) {
        return Err(anyhow!("ตัวเลขทั้งหมดต้องมีความยาวเท่ากันและประกอบด้วยตัวเลขเท่านั้นสำหรับการวิเคราะห์ ML"));
    }

    let mut feature_rows = Vec::with_capacity(numbers_str.len());
    let mut labels = Vec::with_capacity(numbers_str.len());

    for pair in numbers_str.windows(2) {
        let features: Vec<f64> = pair[0].chars().map(|c| (c.to_digit(10).unwrap_or(0)) as f64).collect();
        let label = pair[1].chars().last().unwrap_or('0').to_digit(10).unwrap_or(0) as usize;
        feature_rows.push(features);
        labels.push(label);
    }
    
    if feature_rows.is_empty() { return Err(anyhow!("ไม่สามารถสร้างคู่ฝึกฝนจากข้อมูลได้")); }

    let (n_samples, n_features) = (labels.len(), num_len);
    let flat_features: Vec<f64> = feature_rows.into_iter().flatten().collect();
    let records = Array::from_shape_vec((n_samples, n_features), flat_features)?;
    let targets = Array1::from(labels);
    let dataset = Dataset::new(records, targets);

    let model = DecisionTree::params().fit(&dataset)?;
    
    let last_number_features: Vec<f64> = numbers_str.last().unwrap().chars().map(|c| c.to_digit(10).unwrap() as f64).collect();
    let last_number_array = Array2::from_shape_vec((1, n_features), last_number_features)?;
    let predicted_last_digit = model.predict(&last_number_array);

    Ok(AnalysisResponse {
        statistical_summary: HashMap::from([("โมเดล".to_string(), "Decision Tree Classifier".to_string())]),
        pattern_analysis: HashMap::from([("เป้าหมายการทำนาย".to_string(), serde_json::json!("ตัวเลขสุดท้ายของหมายเลขถัดไป"))]),
        prediction_output: HashMap::from([
            ("PREDICTION".to_string(), serde_json::json!(format!("ตัวเลขใดๆ ที่ลงท้ายด้วย '{}'", predicted_last_digit[0]))),
            ("METHOD".to_string(), serde_json::json!("แมชชีนเลิร์นนิง (Decision Tree)")),
        ]),
        detailed_explanation: HashMap::from([("หลักการ".to_string(), "โมเดล Decision Tree ได้รับการฝึกฝนเพื่อทำนายตัวเลขสุดท้ายของหมายเลขถัดไปโดยพิจารณาจากตัวเลขของหมายเลขก่อนหน้า มันเรียนรู้รูปแบบจากการเปลี่ยนแปลงในอดีต".to_string())]),
    })
}

// --- Method 4: Markov Chain ---
fn run_markov_chain_analysis(numbers_str: &[String]) -> Result<AnalysisResponse> {
    if numbers_str.len() < 2 { return Err(anyhow!("การวิเคราะห์แบบมาร์คอฟเชนต้องการข้อมูลอย่างน้อย 2 จุดข้อมูล")); }

    let mut transitions: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for window in numbers_str.windows(2) {
        *transitions.entry(window[0].clone()).or_default().entry(window[1].clone()).or_default() += 1;
    }

    let last_number = numbers_str.last().unwrap();
    let prediction = transitions.get(last_number).and_then(|possible_next| {
        possible_next.iter().max_by_key(|&(_, count)| count).map(|(num, _)| num.clone())
    }).unwrap_or_else(|| "ไม่พบการเปลี่ยนแปลงในอดีต".to_string());
    
    Ok(AnalysisResponse {
        statistical_summary: HashMap::from([("จำนวนสถานะ (ตัวเลขที่ไม่ซ้ำกัน)".to_string(), transitions.len().to_string())]),
        pattern_analysis: HashMap::new(), // Pattern is implicit in the prediction
        prediction_output: HashMap::from([
            ("PREDICTION".to_string(), serde_json::json!(prediction)),
            ("METHOD".to_string(), serde_json::json!("การวิเคราะห์แบบมาร์คอฟเชน")),
        ]),
        detailed_explanation: HashMap::from([("หลักการ".to_string(), format!("การวิเคราะห์นี้จะคำนวณความน่าจะเป็นในอดีตของการเปลี่ยนจากตัวเลขหนึ่งไปยังตัวเลขถัดไป การทำนายคือตัวเลขที่ตามหลัง '{}' บ่อยที่สุดในอดีต", last_number))]),
    })
}