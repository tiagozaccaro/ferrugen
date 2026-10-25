use sqlx::PgPool;
use uuid::Uuid;

use crate::error::PersistenceError;

#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub password_hash: String,
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id: Uuid,
        email: &str,
        display_name: Option<&str>,
        password_hash: &str,
    ) -> Result<(), PersistenceError> {
        let result = sqlx::query(
            "INSERT INTO users (id, email, display_name, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(email)
        .bind(display_name)
        .bind(password_hash)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err))
                if db_err.constraint() == Some("users_email_key") =>
            {
                Err(PersistenceError::Database("email taken".to_string()))
            }
            Err(e) => Err(PersistenceError::from(e)),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRow>, PersistenceError> {
        let row = sqlx::query_as::<_, (Uuid, String, Option<String>, String)>(
            "SELECT id, email, display_name, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(PersistenceError::from)?;

        Ok(row.map(|(id, email, display_name, password_hash)| UserRow {
            id,
            email,
            display_name,
            password_hash,
        }))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserRow>, PersistenceError> {
        let row = sqlx::query_as::<_, (Uuid, String, Option<String>, String)>(
            "SELECT id, email, display_name, password_hash FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(PersistenceError::from)?;

        Ok(row.map(|(id, email, display_name, password_hash)| UserRow {
            id,
            email,
            display_name,
            password_hash,
        }))
    }
}
