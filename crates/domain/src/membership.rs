#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MembershipStatus {
    Invited,
    Active,
    Removed,
}

#[derive(Debug, Clone)]
pub struct Membership {
    pub user_id: platform_core::UserId,
    pub tenant_id: platform_core::TenantId,
    pub role: crate::role::Role,
    pub status: MembershipStatus,
}
