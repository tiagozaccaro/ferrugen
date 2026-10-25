use sqlx::PgPool;
use std::time::Duration;

pub async fn db_reachable(pool: &PgPool) -> bool {
    tokio::time::timeout(Duration::from_secs(2), async {
        sqlx::query("SELECT 1").fetch_one(pool).await.is_ok()
    })
    .await
    .unwrap_or(false)
}
