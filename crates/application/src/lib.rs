pub mod audit;
pub mod authorize;
pub mod ports;
pub mod tenant_context;
pub mod tenant_service;

use platform_core::{TenantId, UserId};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("user is not a member of the requested tenant")]
    NotMember,
    #[error("forbidden")]
    Forbidden,
    #[error("repository operation failed: {0}")]
    Repository(String),
}

pub use audit::*;
pub use ports::*;
pub use tenant_context::TenantContext;
pub use tenant_service::TenantService;

pub async fn resolve_tenant_context<R: crate::ports::MembershipReader>(
    reader: &R,
    user_id: UserId,
    requested_tenant: TenantId,
) -> Result<TenantContext, ApplicationError> {
    let membership = reader
        .find_membership(user_id, requested_tenant)
        .await?
        .ok_or(ApplicationError::NotMember)?;
    Ok(TenantContext::from_verified_membership(&membership))
}
