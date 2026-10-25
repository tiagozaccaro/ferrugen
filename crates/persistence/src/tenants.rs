use sqlx::PgPool;
use uuid::Uuid;

use crate::error::PersistenceError;

#[derive(Debug, Clone)]
pub struct TenantRow {
    pub id: Uuid,
    pub name: String,
}

pub struct PgTenantRepository {
    pool: PgPool,
}

impl PgTenantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_tenant_with_owner(
        &self,
        tenant_id: Uuid,
        tenant_name: &str,
        user_id: Uuid,
        role: &str,
    ) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(PersistenceError::from)?;
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, $2)")
            .bind(tenant_id)
            .bind(tenant_name)
            .execute(&mut *tx)
            .await
            .map_err(PersistenceError::from)?;
        sqlx::query(
            "INSERT INTO tenant_memberships (user_id, tenant_id, role, status) VALUES ($1, $2, $3, 'active')",
        )
        .bind(user_id)
        .bind(tenant_id)
        .bind(role)
        .execute(&mut *tx)
        .await
        .map_err(PersistenceError::from)?;
        tx.commit().await.map_err(PersistenceError::from)?;
        Ok(())
    }
}
