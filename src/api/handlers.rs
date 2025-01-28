use actix_web::{web, HttpResponse};
use crate::ai::pattern_recognition::PatternRecognition;
use crate::ai::neural_network::NeuralNetwork;
use crate::api::models::{AnalyzeRequest, AnalyzeResponse, PatternResponse, MarketResponse};
use crate::services::market::MarketAnalysis;
use crate::services::market::{TradeVolume, WhaleTransaction};
use uuid::Uuid;
use rand::Rng;

pub async fn analyze_handler(req: web::Json<AnalyzeRequest>) -> impl Responder {
    let chain = req.chain.to_lowercase();
    let data_length = req.data.len();

    if data_length == 0 {
        return HttpResponse::BadRequest().body("Invalid data: Data cannot be empty");
    }

    let result = match chain.as_str() {
        "solana" => "High confidence in bullish trend".to_string(),
        "ethereum" => "Neutral trend with slight bearish pressure".to_string(),
        _ => "Unsupported chain analysis".to_string(),
    };

    let response = AnalyzeResponse {
        result: format!("Chain: {}, Result: {}", chain, result),
    };

    HttpResponse::Ok().json(response)
}


pub async fn analyze_pattern(data: web::Json<String>) -> HttpResponse {
    match PatternRecognition::recognize_pattern(&data) {
        Ok(pattern) => HttpResponse::Ok().json(pattern),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn predict_trend(data: web::Json<String>) -> HttpResponse {
    match NeuralNetwork::run_neural_network(&data) {
        Ok(prediction) => HttpResponse::Ok().json(prediction),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn track_market_volume(data: web::Json<Vec<TradeVolume>>) -> HttpResponse {
    match MarketAnalysis::track_volume("BTC", data.into_inner()) {
        Ok(volume_map) => HttpResponse::Ok().json(volume_map),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn track_whale_activity(data: web::Json<Vec<WhaleTransaction>>) -> HttpResponse {
    match MarketAnalysis::track_whale_transactions(data.into_inner()) {
        Ok(whale_transactions) => HttpResponse::Ok().json(whale_transactions),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn analyze_market_trend(data: web::Json<Vec<WhaleTransaction>>) -> HttpResponse {
    match MarketAnalysis::analyze_market_behavior(data.into_inner()) {
        Ok(market_trend) => HttpResponse::Ok().body(market_trend),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn pattern_handler() -> impl Responder {
    let patterns = vec![
        PatternResponse {
            pattern_id: Uuid::new_v4().to_string(),
            description: "Double top formation detected".to_string(),
            confidence: rand::thread_rng().gen_range(0.7..0.95),
        },
        PatternResponse {
            pattern_id: Uuid::new_v4().to_string(),
            description: "Ascending triangle with breakout".to_string(),
            confidence: rand::thread_rng().gen_range(0.8..0.98),
        },
    ];

    HttpResponse::Ok().json(patterns)
}

pub async fn market_handler() -> impl Responder {
    let trend = if rand::thread_rng().gen_bool(0.6) {
        "Bullish"
    } else {
        "Bearish"
    };
    let whale_wallet = if trend == "Bullish" {
        "0xWHALEBULL123456"
    } else {
        "0xWHALEBEAR654321"
    };

    let response = MarketResponse {
        trend: trend.to_string(),
        volume: rand::thread_rng().gen_range(100000.0..500000.0),
        significant_address: whale_wallet.to_string(),
    };

    HttpResponse::Ok().json(response)
}
