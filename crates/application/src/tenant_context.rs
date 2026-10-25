use domain::Role;
use platform_core::{TenantId, UserId};

pub struct TenantContext {
    user_id: UserId,
    tenant_id: TenantId,
    role: Role,
}

impl TenantContext {
    pub fn new(user_id: UserId, tenant_id: TenantId, role: Role) -> Self {
        Self {
            user_id,
            tenant_id,
            role,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn tenant_id(&self) -> TenantId {
        self.tenant_id
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub(crate) fn from_verified_membership(membership: &domain::Membership) -> Self {
        Self {
            user_id: membership.user_id,
            tenant_id: membership.tenant_id,
            role: membership.role,
        }
    }
}
