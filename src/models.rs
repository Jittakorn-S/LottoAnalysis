use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// --- Core Data Structures ---

/// Represents a single lottery result, generic for any country.
/// Prize1 and Prize2 are used generically to accommodate different lottery formats.
/// For Thai: prize1 = First Prize, prize2 = Last 2 Digits.
/// For Laos: prize1 = 3-Digit Prize, prize2 = 2-Digit Prize.
#[derive(Serialize, Clone, Debug)]
pub struct LottoResult {
    #[serde(rename = "Draw Date")]
    pub draw_date: String,
    #[serde(rename = "Prize 1")]
    pub prize1: String,
    #[serde(rename = "Prize 2")]
    pub prize2: String,
}

/// Holds the state of the current or last scraping task. This is shared across threads.
#[derive(Serialize, Clone)]
pub struct TaskStatus {
    pub is_running: bool,
    pub lotto_type: Option<String>,
    pub progress: Vec<String>,
    pub results: Vec<LottoResult>,
}

impl TaskStatus {
    /// Creates a new, default task status.
    fn new() -> Self {
        TaskStatus {
            is_running: false,
            lotto_type: None,
            progress: Vec::new(),
            results: Vec::new(),
        }
    }
}

/// Shared application state accessible by all API handlers.
/// The Mutex ensures safe concurrent access to the task status.
pub struct AppState {
    pub task_status: Mutex<TaskStatus>,
}

impl AppState {
    /// Creates a new AppState.
    pub fn new() -> Self {
        AppState {
            task_status: Mutex::new(TaskStatus::new()),
        }
    }
}


// --- API Request & Response Structs ---

/// Type of lottery to scrape, chosen by the user in the frontend.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LottoType {
    Thai,
    Laos,
}

/// Allows LottoType to be easily converted to a string for display.
impl std::fmt::Display for LottoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LottoType::Thai => write!(f, "ไทย"),
            LottoType::Laos => write!(f, "ลาว"),
        }
    }
}

/// Request from the frontend to start a new scraping task.
#[derive(Deserialize)]
pub struct StartScrapeRequest {
    pub lotto_type: LottoType,
}

/// The analysis method chosen by the user.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisMethod {
    ComprehensiveStatistics,
    Numerology,
    MlDecisionTree,
    MarkovChain,
}

/// Request from the frontend to perform an analysis.
#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub numbers: Vec<String>,
    pub method: AnalysisMethod,
}

/// The structure of the JSON response for a successful analysis request.
#[derive(Serialize)]
pub struct AnalysisResponse {
    pub statistical_summary: std::collections::HashMap<String, String>,
    pub pattern_analysis: std::collections::HashMap<String, serde_json::Value>,
    pub prediction_output: std::collections::HashMap<String, serde_json::Value>,
    pub detailed_explanation: std::collections::HashMap<String, String>,
}