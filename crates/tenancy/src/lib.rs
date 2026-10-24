//! Tenant context and membership use cases.
//!
//! Persistence adapters must scope every tenant-owned query by a verified
//! `TenantContext`. Never accept a tenant ID as sufficient authorization.
pub use application::{ApplicationError, MembershipReader, TenantContext, resolve_tenant_context};
pub use domain::{Membership, Tenant};
pub use platform_core::{TenantId, UserId};
