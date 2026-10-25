use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub async fn health_live() -> Json<Value> {
    Json(json!({"status":"ok","version":env!("CARGO_PKG_VERSION")}))
}

pub async fn health_ready() -> (axum::http::StatusCode, Json<Value>) {
    (
        axum::http::StatusCode::OK,
        Json(json!({"status":"ready","version":"0.1.0"})),
    )
}

pub fn routes() -> Router {
    Router::new()
        .route("/health/live", get(health_live))
        .route("/health/ready", get(health_ready))
}
