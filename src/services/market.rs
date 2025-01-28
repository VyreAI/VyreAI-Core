use log::{info, error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeVolume {
    pub symbol: String,
    pub volume: f64,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhaleTransaction {
    pub wallet_address: String,
    pub amount_transferred: f64,
    pub direction: String, 
    pub timestamp: String,
}

pub struct MarketAnalysis;

impl MarketAnalysis {
    pub fn track_volume(symbol: &str, volume_data: Vec<TradeVolume>) -> Result<HashMap<String, f64>, String> {
        let mut volume_map = HashMap::new();
        
        for data in volume_data {
            if data.symbol == symbol {
                info!("Tracking volume for {}: {} at {}", symbol, data.volume, data.timestamp);
                let entry = volume_map.entry(symbol.to_string()).or_insert(0.0);
                *entry += data.volume;
            }
        }
        
        if volume_map.is_empty() {
            error!("No volume data found for symbol {}", symbol);
            return Err(format!("No data found for symbol {}", symbol));
        }

        Ok(volume_map)
    }

    pub fn track_whale_transactions(transactions: Vec<WhaleTransaction>) -> Result<Vec<WhaleTransaction>, String> {
        let mut whale_transactions = Vec::new();
        
        for transaction in transactions {
            if transaction.amount_transferred > 1000.0 {
                info!("Whale transaction detected: {} transferred {} at {}", transaction.wallet_address, transaction.amount_transferred, transaction.timestamp);
                whale_transactions.push(transaction);
            }
        }
        
        if whale_transactions.is_empty() {
            error!("No significant whale transactions found.");
            return Err("No whale transactions detected".to_string());
        }

        Ok(whale_transactions)
    }


    pub fn analyze_market_behavior(wallets_data: Vec<WhaleTransaction>) -> Result<String, String> {
        let mut total_in = 0.0;
        let mut total_out = 0.0;

        for data in wallets_data {
            if data.direction == "in" {
                total_in += data.amount_transferred;
            } else if data.direction == "out" {
                total_out += data.amount_transferred;
            }
        }

        info!("Total market inflows: {}, Total outflows: {}", total_in, total_out);

        if total_in > total_out {
            Ok("Bullish market trend detected based on wallet inflows.".to_string())
        } else if total_out > total_in {
            Ok("Bearish market trend detected based on wallet outflows.".to_string())
        } else {
            Ok("Neutral market detected.".to_string())
        }
    }
}
