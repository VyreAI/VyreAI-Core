use actix_web::web;
use crate::api::handlers::{
    analyze_handler, pattern_handler, market_handler,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/analyze", web::post().to(analyze_handler))
            .route("/pattern", web::get().to(pattern_handler))
            .route("/market", web::get().to(market_handler)),
    );
}
