pub mod auth;
pub mod tenancy;
use persistence::{
    PgMembershipRepository, PgPool, PgSessionRepository, PgTenantRepository, PgUserRepository,
};

pub fn user_repository(pool: PgPool) -> PgUserRepository {
    PgUserRepository::new(pool)
}

pub fn tenant_repository(pool: PgPool) -> PgTenantRepository {
    PgTenantRepository::new(pool)
}

pub fn membership_repository(pool: PgPool) -> PgMembershipRepository {
    PgMembershipRepository::new(pool)
}

pub fn session_repository(pool: PgPool) -> PgSessionRepository {
    PgSessionRepository::new(pool)
}
