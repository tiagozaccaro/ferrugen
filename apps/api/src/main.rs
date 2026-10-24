use axum::{routing::get, Json, Router};
use contracts::HealthResponse;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

async fn live() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".to_owned(), version: env!("CARGO_PKG_VERSION").to_owned() })
}

async fn ready() -> (axum::http::StatusCode, Json<HealthResponse>) {
    // Replace with real dependency checks when PostgreSQL is wired.
    (axum::http::StatusCode::SERVICE_UNAVAILABLE, Json(HealthResponse {
        status: "not_ready_database_not_wired".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let app = Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
        .layer(TraceLayer::new_for_http());

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port: u16 = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_owned()).parse()?;
    let address: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = TcpListener::bind(address).await?;
    tracing::info!(%address, "API listening");
    axum::serve(listener, app).await?;
    Ok(())
}
