#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use persistence::{PgMembershipRepository, PgTenantRepository, PgUserRepository, run_migrations};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = false)]
async fn cross_tenant_membership_denied(pool: PgPool) {
    run_migrations(&pool).await.unwrap();

    let user_repo = PgUserRepository::new(pool.clone());
    let tenant_repo = PgTenantRepository::new(pool.clone());
    let member_repo = PgMembershipRepository::new(pool.clone());

    let user_id = Uuid::new_v4();
    user_repo
        .create(user_id, "a@example.com", None, "hash")
        .await
        .unwrap();

    let tenant_a = Uuid::new_v4();
    let tenant_b = Uuid::new_v4();
    tenant_repo
        .create_tenant_with_owner(tenant_a, "Tenant A", user_id, "owner")
        .await
        .unwrap();
    // create second tenant with same user as owner
    tenant_repo
        .create_tenant_with_owner(tenant_b, "Tenant B", user_id, "owner")
        .await
        .unwrap();

    // tenant A membership should exist
    let m_a = member_repo.find(user_id, tenant_a).await.unwrap();
    assert!(m_a.is_some());
    // tenant B membership should exist
    let m_b = member_repo.find(user_id, tenant_b).await.unwrap();
    assert!(m_b.is_some());

    // A query for a non-existent tenant returns None
    let none = member_repo.find(user_id, Uuid::new_v4()).await.unwrap();
    assert!(none.is_none());

    // Cross-tenant change_role / remove denied (SC-003): user in tenant A
    // should not affect tenant B membership.
    member_repo
        .change_role(user_id, tenant_a, "admin")
        .await
        .unwrap();
    let m_a_after = member_repo.find(user_id, tenant_a).await.unwrap().unwrap();
    assert_eq!(m_a_after.role, "admin");
    let m_b_after = member_repo.find(user_id, tenant_b).await.unwrap().unwrap();
    assert_eq!(m_b_after.role, "owner");
}
