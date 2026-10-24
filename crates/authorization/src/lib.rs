use platform_core::TenantId;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    TenantRead,
    TenantManage,
    MemberRead,
    MemberInvite,
    MemberRemove,
    AuditRead,
    SettingsManage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grant {
    pub tenant_id: TenantId,
    pub permission: Permission,
}

pub fn is_allowed(grants: &[Grant], tenant_id: TenantId, permission: &Permission) -> bool {
    grants.iter().any(|grant| grant.tenant_id == tenant_id && &grant.permission == permission)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthorizationError {
    #[error("permission denied")]
    Denied,
}
