use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

use crate::error::PersistenceError;

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, PersistenceError> {
    if database_url.trim().is_empty() {
        return Err(PersistenceError::Configuration(
            "DATABASE_URL is required".to_string(),
        ));
    }
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
        .map_err(PersistenceError::from)
}
