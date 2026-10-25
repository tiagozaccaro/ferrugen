use sqlx::PgPool;

use crate::error::PersistenceError;

pub async fn run_migrations(pool: &PgPool) -> Result<(), PersistenceError> {
    sqlx::migrate!("../../migrations")
        .run(pool)
        .await
        .map_err(|e| PersistenceError::Database(e.to_string()))
}
