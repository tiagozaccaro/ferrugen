use axum::{Json, Router, extract::State, routing::get};
use contracts::HealthResponse;
use std::sync::Arc;


use crate::state::{AppState, MigrationState};

pub async fn health_live() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

pub async fn health_ready(
    State(state): State<Arc<AppState>>,
) -> (axum::http::StatusCode, Json<HealthResponse>) {
    // Bounded DB check (2-second bound per contracts/health.md)
    let db_ok = persistence::health_check::db_reachable(&state.db).await;

    if !db_ok {
        return (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "not_ready_database_unavailable".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            }),
        );
    }

    match state.migration_state {
        Some(MigrationState::Applied) => (
            axum::http::StatusCode::OK,
            Json(HealthResponse {
                status: "ready".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            }),
        ),
        _ => (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "not_ready_migrations_failed".to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
            }),
        ),
    }
}

#[allow(dead_code)]
pub fn routes() -> Router {
    Router::new().route("/health/live", get(health_live))
}
