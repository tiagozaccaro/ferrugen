//! PostgreSQL adapter for the platform.
//! SQLx is isolated here; domain crates must not depend on it.

pub const DATABASE_KIND: &str = "postgresql";

pub mod error;
pub mod health_check;
pub mod memberships;
pub mod migrations;
pub mod pool;
pub mod sessions;
pub mod tenants;
pub mod users;

pub use error::PersistenceError;
pub use memberships::{MembershipRow, PgMembershipRepository};
pub use migrations::run_migrations;
pub use pool::create_pool;
pub use sessions::{PgSessionRepository, SessionRow};
pub use sqlx::PgPool;
pub use tenants::{PgTenantRepository, TenantRow};
pub use users::{PgUserRepository, UserRow};
