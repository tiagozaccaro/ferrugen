use axum::{Router, routing::get};
use persistence::{create_pool, run_migrations};
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

mod health;
mod state;
use state::{AppState, MigrationState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set at startup (contracts/health.md)")?;
    let pool = create_pool(&database_url).await?;

    // Record migration state (start before serve, per contract)
    let migration_state = match run_migrations(&pool).await {
        Ok(_) => Some(MigrationState::Applied),
        Err(_) => Some(MigrationState::Failed),
    };

    let state = Arc::new(AppState {
        db: Arc::new(pool),
        migration_state,
    });

    let app = Router::new()
        .route("/health/live", get(health::health_live))
        .route("/health/ready", get(health::health_ready))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_owned())
        .parse()?;
    let address: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "API listening");
    axum::serve(listener, app).await?;
    Ok(())
}
