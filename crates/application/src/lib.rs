use domain::Membership;
use platform_core::{TenantId, UserId};
use thiserror::Error;

/// Verified request context. Construct this only after checking the authenticated
/// user's membership; never construct it directly from an untrusted request field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TenantContext {
    user_id: UserId,
    tenant_id: TenantId,
}

impl TenantContext {
    pub(crate) fn from_verified_membership(membership: &Membership) -> Self {
        Self { user_id: membership.user_id, tenant_id: membership.tenant_id }
    }
    pub fn user_id(&self) -> UserId { self.user_id }
    pub fn tenant_id(&self) -> TenantId { self.tenant_id }
}

pub trait MembershipReader {
    fn membership_for(&self, user_id: UserId, tenant_id: TenantId) -> Result<Option<Membership>, ApplicationError>;
}

pub fn resolve_tenant_context<R: MembershipReader>(
    reader: &R,
    user_id: UserId,
    requested_tenant: TenantId,
) -> Result<TenantContext, ApplicationError> {
    let membership = reader.membership_for(user_id, requested_tenant)?
        .ok_or(ApplicationError::NotMember)?;
    Ok(TenantContext::from_verified_membership(&membership))
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    #[error("user is not a member of the requested tenant")]
    NotMember,
    #[error("repository operation failed: {0}")]
    Repository(String),
}
