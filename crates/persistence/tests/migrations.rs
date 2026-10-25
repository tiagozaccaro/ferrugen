#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use persistence::run_migrations;
use sqlx::PgPool;

#[sqlx::test(migrations = false)]
async fn fresh_migration_applies(pool: PgPool) {
    run_migrations(&pool).await.unwrap();
    // second run is a no-op
    run_migrations(&pool).await.unwrap();
}
