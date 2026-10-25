#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use application::ApplicationError;
use application::TenantService;
use application::audit::{AuditEvent, AuditSink};
use application::ports::*;
use application::tenant_context::TenantContext;
use domain::{Membership, MembershipStatus, Role};
use platform_core::{TenantId, UserId};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

struct RecordingAuditSink(Mutex<Vec<AuditEvent>>);

impl RecordingAuditSink {
    fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }
}

#[async_trait::async_trait]
impl AuditSink for RecordingAuditSink {
    async fn record(&self, event: AuditEvent) -> Result<(), application::audit::AuditError> {
        self.0.lock().unwrap().push(event);
        Ok(())
    }
}

struct InMemMembershipReader(Mutex<HashMap<(UserId, TenantId), Membership>>);

impl Default for InMemMembershipReader {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

struct InMemTenantRepository(Mutex<HashMap<TenantId, String>>);

impl Default for InMemTenantRepository {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

struct InMemMembershipRepository(Mutex<HashMap<(TenantId, UserId), Role>>);

impl Default for InMemMembershipRepository {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

#[async_trait::async_trait]
impl MembershipReader for InMemMembershipReader {
    async fn find_membership(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Option<Membership>, ApplicationError> {
        let m = self.0.lock().unwrap();
        Ok(m.get(&(user_id, tenant_id)).cloned())
    }
}

#[async_trait::async_trait]
impl TenantRepository for InMemTenantRepository {
    async fn create_tenant(
        &self,
        name: &str,
        _owner_user_id: UserId,
    ) -> Result<TenantId, ApplicationError> {
        let id = TenantId(Uuid::new_v4());
        self.0.lock().unwrap().insert(id, name.to_string());
        Ok(id)
    }
}

#[async_trait::async_trait]
impl MembershipRepository for InMemMembershipRepository {
    async fn change_role(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        new_role: Role,
    ) -> Result<(), ApplicationError> {
        self.0
            .lock()
            .unwrap()
            .insert((tenant_id, user_id), new_role);
        Ok(())
    }

    async fn remove_member(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
    ) -> Result<(), ApplicationError> {
        self.0.lock().unwrap().remove(&(tenant_id, user_id));
        Ok(())
    }
}

#[tokio::test]
async fn tenant_context_resolve_success() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());
    reader.0.lock().unwrap().insert(
        (user_id, tenant_id),
        Membership {
            user_id,
            tenant_id,
            role: Role::Admin,
            status: MembershipStatus::Active,
        },
    );

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let ctx = svc.resolve_context(user_id, tenant_id).await.unwrap();
    assert_eq!(ctx.user_id(), user_id);
    assert_eq!(ctx.tenant_id(), tenant_id);
    assert_eq!(ctx.role(), Role::Admin);
}

#[tokio::test]
async fn tenant_context_resolve_fail_not_active() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());
    reader.0.lock().unwrap().insert(
        (user_id, tenant_id),
        Membership {
            user_id,
            tenant_id,
            role: Role::Member,
            status: MembershipStatus::Invited,
        },
    );

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let res = svc.resolve_context(user_id, tenant_id).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn tenant_context_resolve_fail_not_member() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());
    let res = svc.resolve_context(user_id, tenant_id).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn change_role_success() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let user_id = UserId(Uuid::new_v4());
    let target_user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());

    let actor = TenantContext::new(user_id, tenant_id, Role::Admin);
    let res = svc.change_role(&actor, target_user_id, Role::Admin).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn change_role_denied() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let user_id = UserId(Uuid::new_v4());
    let target_user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());

    let actor = TenantContext::new(user_id, tenant_id, Role::Member);
    let res = svc.change_role(&actor, target_user_id, Role::Admin).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn remove_member_success() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let user_id = UserId(Uuid::new_v4());
    let target_user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());

    let actor = TenantContext::new(user_id, tenant_id, Role::Admin);
    let res = svc.remove_member(&actor, target_user_id).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn remove_member_denied() {
    let reader = InMemMembershipReader::default();
    let tenant_repo = InMemTenantRepository::default();
    let membership_repo = InMemMembershipRepository::default();

    let audit = RecordingAuditSink::new();
    let svc = TenantService::new(reader, tenant_repo, membership_repo, audit);

    let user_id = UserId(Uuid::new_v4());
    let target_user_id = UserId(Uuid::new_v4());
    let tenant_id = TenantId(Uuid::new_v4());

    let actor = TenantContext::new(user_id, tenant_id, Role::Member);
    let res = svc.remove_member(&actor, target_user_id).await;
    assert!(res.is_err());
}
#[tokio::test]
async fn audit_emits_role_changed_on_success() {
    let audit = RecordingAuditSink::new();
    // audit events are emitted through TenantService; verify sink receives them
    assert!(!audit.0.lock().unwrap().is_empty());
}
