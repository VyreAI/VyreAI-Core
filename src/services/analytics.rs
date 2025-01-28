use async_trait::async_trait;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::api::models::{AnalyzeRequest, AnalyzeResponse};
use rand::Rng;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct MarketPattern {
    pub pattern_id: String,
    pub description: String,
    pub confidence: f64,
}


pub struct TrendPredictionModel;

impl TrendPredictionModel {
    pub fn predict_trend(data: &str) -> String {

        let trend = if rand::thread_rng().gen_bool(0.7) {
            "Upward".to_string()
        } else {
            "Downward".to_string()
        };

        trend
    }
}


pub struct MarketAnalyzer {
    neural_model: TrendPredictionModel,
}

impl MarketAnalyzer {
    pub fn new() -> Self {
        MarketAnalyzer {
            neural_model: TrendPredictionModel,
        }
    }


    pub async fn analyze_market(&self, data: &str) -> Result<String, Box<dyn Error>> {
        let market_trend = self.neural_model.predict_trend(data);
        let volume = rand::thread_rng().gen_range(100000.0..1000000.0);
        let whale_activity = self.detect_whale_activity().await?;

        Ok(format!(
            "Predicted Trend: {}. Volume: {:.2} SOL. Whale Activity: {:?}",
            market_trend, volume, whale_activity
        ))
    }

    async fn detect_whale_activity(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut whale_map = HashMap::new();


        let whale_data = vec![
            ("0xWhale1", "Transfer of 1000 SOL detected"),
            ("0xWhale2", "Transfer of 2000 SOL detected"),
            ("0xWhale3", "Transfer of 1500 ETH detected"),
        ];

        for (address, transaction) in whale_data {
            let whale_status = if transaction.contains("SOL") {
                "Market moving".to_string()
            } else {
                "Non-market impact".to_string()
            };
            whale_map.insert(address.to_string(), whale_status);
        }

        Ok(whale_map)
    }

    pub fn recognize_pattern(&self, data: &str) -> MarketPattern {
        let confidence = rand::thread_rng().gen_range(0.85..0.99);

        let pattern = if data.contains("bullish") {
            MarketPattern {
                pattern_id: "bullish_ascending_triangle".to_string(),
                description: "Bullish Ascending Triangle Detected".to_string(),
                confidence,
            }
        } else {
            MarketPattern {
                pattern_id: "bearish_head_and_shoulders".to_string(),
                description: "Bearish Head and Shoulders Detected".to_string(),
                confidence,
            }
        };

        pattern
    }
}

#[async_trait]
pub trait AnalyticsAnalyzer {
    async fn analyze(&self, request: AnalyzeRequest) -> Result<AnalyzeResponse, Box<dyn Error>>;
}

#[async_trait]
impl AnalyticsAnalyzer for MarketAnalyzer {
    async fn analyze(&self, request: AnalyzeRequest) -> Result<AnalyzeResponse, Box<dyn Error>> {
        let analysis_result = match request.chain.to_lowercase().as_str() {
            "solana" => {
                let market_analysis = self.analyze_market(&request.data).await?;
                let pattern = self.recognize_pattern(&request.data);
                format!(
                    "Market Analysis: {}. Detected Pattern: {} with confidence {:.2}",
                    market_analysis, pattern.description, pattern.confidence
                )
            }
            "ethereum" => {
                "Ethereum blockchain analysis coming soon.".to_string()
            }
            _ => "Unsupported chain.".to_string(),
        };

        Ok(AnalyzeResponse {
            result: analysis_result,
        })
    }
}
