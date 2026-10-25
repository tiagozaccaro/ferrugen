use crate::ApplicationError;
use platform_core::{TenantId, UserId};

#[async_trait::async_trait]
pub trait MembershipReader: Send + Sync {
    async fn find_membership(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Option<domain::Membership>, ApplicationError>;
}

#[async_trait::async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create_tenant(
        &self,
        name: &str,
        owner_user_id: UserId,
    ) -> Result<TenantId, ApplicationError>;
}

#[async_trait::async_trait]
pub trait MembershipRepository: Send + Sync {
    async fn change_role(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        new_role: domain::Role,
    ) -> Result<(), ApplicationError>;
    async fn remove_member(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
    ) -> Result<(), ApplicationError>;
}
