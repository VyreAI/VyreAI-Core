use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub chain: String,
    pub data: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatternResponse {
    pub pattern_id: String,
    pub description: String,
    pub confidence: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MarketResponse {
    pub trend: String,
    pub volume: f64,
    pub significant_address: String,
}
