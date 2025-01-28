use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;

#[tokio::test]
async fn test_market_analysis() {

    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let client = Client::new();
    let url = format!("{}/api/v1/market-analysis", base_url);


    let request_data = json!({
        "chain": "solana",
        "data": "bullish market data"
    });


    let response = client
        .post(&url)
        .json(&request_data)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send request");


    assert_eq!(response.status(), 200);


    let response_body: Value = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert!(response_body["result"].is_string());
    assert!(response_body["result"].as_str().unwrap().contains("Predicted Trend"));

    println!("Market analysis response: {:?}", response_body);
}

#[tokio::test]
async fn test_pattern_recognition() {
    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let client = Client::new();
    let url = format!("{}/api/v1/pattern-recognition", base_url);


    let request_data = json!({
        "chain": "solana",
        "data": "bullish ascending triangle pattern"
    });

    let response = client
        .post(&url)
        .json(&request_data)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send request");

    
    assert_eq!(response.status(), 200);

    let response_body: Value = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert!(response_body["pattern_id"].is_string());
    assert!(response_body["confidence"].is_number());
    assert!(response_body["description"].is_string());

    println!("Pattern recognition response: {:?}", response_body);
}

#[tokio::test]
async fn test_whale_tracking() {
    let base_url = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let client = Client::new();
    let url = format!("{}/api/v1/whale-tracking", base_url);

    let request_data = json!({
        "chain": "solana",
        "data": "track large transactions"
    });

    let response = client
        .post(&url)
        .json(&request_data)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let response_body: Value = response
        .json()
        .await
        .expect("Failed to parse response body");

    assert!(response_body["whale_activity"].is_object());


    println!("Whale tracking response: {:?}", response_body);
}
