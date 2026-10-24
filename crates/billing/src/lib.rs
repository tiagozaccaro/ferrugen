use platform_core::TenantId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entitlement {
    pub tenant_id: TenantId,
    pub capability: String,
    pub limit: Option<u64>,
}

/// Entitlements are usable without a payment provider. Billing integrations
/// may update subscriptions, but authorization must consume entitlements only.
pub trait EntitlementReader {
    fn permits(&self, tenant_id: TenantId, capability: &str) -> bool;
}
