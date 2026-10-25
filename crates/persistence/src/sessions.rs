use sqlx::PgPool;
use uuid::Uuid;

use crate::error::PersistenceError;

#[derive(Debug, Clone)]
pub struct SessionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct PgSessionRepository {
    pool: PgPool,
}

impl PgSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        token_hash: &[u8],
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), PersistenceError> {
        sqlx::query(
            "INSERT INTO sessions (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(PersistenceError::from)?;
        Ok(())
    }

    pub async fn find_by_token_hash(
        &self,
        token_hash: &[u8],
    ) -> Result<Option<SessionRow>, PersistenceError> {
        let row = sqlx::query_as::<_, (Uuid, Uuid, Vec<u8>, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>(
            "SELECT id, user_id, token_hash, expires_at, revoked_at FROM sessions WHERE token_hash = $1",
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await
        .map_err(PersistenceError::from)?;
        Ok(row.map(
            |(id, user_id, token_hash, expires_at, revoked_at)| SessionRow {
                id,
                user_id,
                token_hash,
                expires_at,
                revoked_at,
            },
        ))
    }

    pub async fn revoke(&self, id: Uuid) -> Result<(), PersistenceError> {
        sqlx::query("UPDATE sessions SET revoked_at = now() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::from)?;
        Ok(())
    }

    pub async fn delete_expired(&self) -> Result<u64, PersistenceError> {
        let res = sqlx::query("DELETE FROM sessions WHERE expires_at < now()")
            .execute(&self.pool)
            .await
            .map_err(PersistenceError::from)?;
        Ok(res.rows_affected())
    }
}
