use sqlx::PgPool;
use uuid::Uuid;

use crate::error::PersistenceError;

#[derive(Debug, Clone)]
pub struct MembershipRow {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub status: String,
}

pub struct PgMembershipRepository {
    pool: PgPool,
}

impl PgMembershipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<Option<MembershipRow>, PersistenceError> {
        let row = sqlx::query_as::<_, (Uuid, Uuid, String, String)>(
            "SELECT user_id, tenant_id, role, status FROM tenant_memberships WHERE user_id = $1 AND tenant_id = $2",
        )
        .bind(user_id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(PersistenceError::from)?;
        Ok(row.map(|(user_id, tenant_id, role, status)| MembershipRow {
            user_id,
            tenant_id,
            role,
            status,
        }))
    }

    pub async fn change_role(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        new_role: &str,
    ) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(PersistenceError::from)?;
        // lock row
        let current: Option<(String,)> = sqlx::query_as(
            "SELECT role FROM tenant_memberships WHERE user_id = $1 AND tenant_id = $2 FOR UPDATE",
        )
        .bind(user_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(PersistenceError::from)?;

        let current_role = match current {
            Some((r,)) => r,
            None => {
                return Err(PersistenceError::Database(
                    "membership not found".to_string(),
                ));
            }
        };

        // last-owner guard
        if current_role == "owner" && new_role != "owner" {
            let owner_count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM tenant_memberships WHERE tenant_id = $1 AND role = 'owner' AND status = 'active' FOR UPDATE",
            )
            .bind(tenant_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(PersistenceError::from)?;
            if owner_count.0 <= 1 {
                return Err(PersistenceError::Database(
                    "last owner cannot be demoted".to_string(),
                ));
            }
        }

        sqlx::query(
            "UPDATE tenant_memberships SET role = $1, updated_at = now() WHERE user_id = $2 AND tenant_id = $3",
        )
        .bind(new_role)
        .bind(user_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(PersistenceError::from)?;
        tx.commit().await.map_err(PersistenceError::from)?;
        Ok(())
    }

    pub async fn remove_member(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
    ) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(PersistenceError::from)?;
        let current: Option<(String,)> = sqlx::query_as(
            "SELECT role FROM tenant_memberships WHERE user_id = $1 AND tenant_id = $2 FOR UPDATE",
        )
        .bind(user_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(PersistenceError::from)?;

        let current_role = match current {
            Some((r,)) => r,
            None => {
                return Err(PersistenceError::Database(
                    "membership not found".to_string(),
                ));
            }
        };

        if current_role == "owner" {
            let owner_count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM tenant_memberships WHERE tenant_id = $1 AND role = 'owner' AND status = 'active' FOR UPDATE",
            )
            .bind(tenant_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(PersistenceError::from)?;
            if owner_count.0 <= 1 {
                return Err(PersistenceError::Database(
                    "last owner cannot be removed".to_string(),
                ));
            }
        }

        sqlx::query(
            "UPDATE tenant_memberships SET status = 'removed', updated_at = now() WHERE user_id = $1 AND tenant_id = $2",
        )
        .bind(user_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(PersistenceError::from)?;
        tx.commit().await.map_err(PersistenceError::from)?;
        Ok(())
    }
}
