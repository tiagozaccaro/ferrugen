use domain::{Membership, Tenant};
use platform_core::{TenantId, UserId};

pub fn sample_membership(user_id: UserId, tenant_id: TenantId) -> Membership {
    Membership { user_id, tenant_id, role: "member".to_owned() }
}

pub fn sample_tenant(name: &str) -> Result<Tenant, domain::DomainError> {
    Tenant::new(TenantId::new(), name.to_owned())
}
