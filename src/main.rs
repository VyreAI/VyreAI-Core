use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

async fn health_check() -> impl Responder {
    "LyzerAI-Core API is running"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    println!("Starting LyzerAI-Core API at http://{}/", address);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind(address)?
    .run()
    .await
}
