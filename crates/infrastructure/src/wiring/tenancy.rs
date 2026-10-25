use application::ApplicationError;
use application::ports::{MembershipReader, MembershipRepository, TenantRepository};
use domain::{Membership, MembershipStatus, Role};
use persistence::{PgMembershipRepository, PgTenantRepository};
use platform_core::{TenantId, UserId};
use uuid::Uuid;

pub struct PgMembershipReaderAdapter(pub PgMembershipRepository);
pub struct PgTenantRepositoryAdapter(pub PgTenantRepository);
pub struct PgMembershipRepositoryAdapter(pub PgMembershipRepository);

#[async_trait::async_trait]
impl MembershipReader for PgMembershipReaderAdapter {
    async fn find_membership(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Option<Membership>, ApplicationError> {
        let row = self
            .0
            .find(user_id.0, tenant_id.0)
            .await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        Ok(row.map(|r| Membership {
            user_id: UserId(r.user_id),
            tenant_id: TenantId(r.tenant_id),
            role: match r.role.as_str() {
                "owner" => Role::Owner,
                "admin" => Role::Admin,
                "viewer" => Role::Viewer,
                _ => Role::Member,
            },
            status: match r.status.as_str() {
                "invited" => MembershipStatus::Invited,
                "removed" => MembershipStatus::Removed,
                _ => MembershipStatus::Active,
            },
        }))
    }
}

#[async_trait::async_trait]
impl TenantRepository for PgTenantRepositoryAdapter {
    async fn create_tenant(
        &self,
        name: &str,
        owner_user_id: UserId,
    ) -> Result<TenantId, ApplicationError> {
        let id = Uuid::new_v4();
        self.0
            .create_tenant_with_owner(id, name, owner_user_id.0, "owner")
            .await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;
        Ok(TenantId(id))
    }
}

#[async_trait::async_trait]
impl MembershipRepository for PgMembershipRepositoryAdapter {
    async fn change_role(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        new_role: Role,
    ) -> Result<(), ApplicationError> {
        self.0
            .change_role(user_id.0, tenant_id.0, new_role.as_str())
            .await
            .map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    async fn remove_member(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
    ) -> Result<(), ApplicationError> {
        self.0
            .remove_member(user_id.0, tenant_id.0)
            .await
            .map_err(|e| ApplicationError::Repository(e.to_string()))
    }
}
