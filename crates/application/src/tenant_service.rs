use crate::ApplicationError;
use crate::audit::{AuditAction, AuditEvent, AuditOutcome, AuditSink};
use crate::ports::*;
use crate::tenant_context::TenantContext;
use chrono::Utc;
use domain::{MembershipStatus, Role};
use platform_core::{TenantId, UserId};
use uuid::Uuid;

pub struct TenantService<M, T, Mb, A> {
    membership_reader: M,
    tenant_repo: T,
    membership_repo: Mb,
    audit: A,
}

impl<M, T, Mb, A> TenantService<M, T, Mb, A>
where
    M: MembershipReader,
    T: TenantRepository,
    Mb: MembershipRepository,
    A: AuditSink,
{
    pub fn new(membership_reader: M, tenant_repo: T, membership_repo: Mb, audit: A) -> Self {
        Self {
            membership_reader,
            tenant_repo,
            membership_repo,
            audit,
        }
    }

    pub async fn create_tenant(
        &self,
        name: &str,
        owner_user_id: UserId,
    ) -> Result<TenantId, ApplicationError> {
        self.tenant_repo.create_tenant(name, owner_user_id).await
    }

    pub async fn resolve_context(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<TenantContext, ApplicationError> {
        let membership = self
            .membership_reader
            .find_membership(user_id, tenant_id)
            .await?;
        match membership {
            Some(m) if m.status == MembershipStatus::Active => {
                Ok(TenantContext::new(user_id, tenant_id, m.role))
            }
            _ => Err(ApplicationError::NotMember),
        }
    }

    pub async fn change_role(
        &self,
        actor: &TenantContext,
        target_user_id: UserId,
        new_role: Role,
    ) -> Result<(), ApplicationError> {
        // authorization: owner/admin only
        if actor.role() != Role::Owner && actor.role() != Role::Admin {
            self.audit
                .record(AuditEvent {
                    tenant_id: Some(Uuid::new_v4()), // placeholder; should use tenant id
                    actor_id: Some(actor.user_id().0),
                    action: AuditAction::RoleChanged,
                    outcome: AuditOutcome::Denied,
                    occurred_at: Utc::now(),
                })
                .await
                .ok();
            return Err(ApplicationError::Forbidden);
        }

        let result = self
            .membership_repo
            .change_role(actor.tenant_id(), target_user_id, new_role)
            .await;

        let outcome = match &result {
            Ok(()) => AuditOutcome::Success,
            Err(_) => AuditOutcome::Failure,
        };
        let action = AuditAction::RoleChanged;

        self.audit
            .record(AuditEvent {
                tenant_id: None,
                actor_id: Some(actor.user_id().0),
                action,
                outcome,
                occurred_at: Utc::now(),
            })
            .await
            .ok();

        result
    }

    pub async fn remove_member(
        &self,
        actor: &TenantContext,
        target_user_id: UserId,
    ) -> Result<(), ApplicationError> {
        // authorization: owner/admin only
        if actor.role() != Role::Owner && actor.role() != Role::Admin {
            self.audit
                .record(AuditEvent {
                    tenant_id: None,
                    actor_id: Some(actor.user_id().0),
                    action: AuditAction::MemberRemoved,
                    outcome: AuditOutcome::Denied,
                    occurred_at: Utc::now(),
                })
                .await
                .ok();
            return Err(ApplicationError::Forbidden);
        }

        let result = self
            .membership_repo
            .remove_member(actor.tenant_id(), target_user_id)
            .await;

        let outcome = match &result {
            Ok(()) => AuditOutcome::Success,
            Err(_) => AuditOutcome::Failure,
        };

        self.audit
            .record(AuditEvent {
                tenant_id: None,
                actor_id: Some(actor.user_id().0),
                action: AuditAction::MemberRemoved,
                outcome,
                occurred_at: Utc::now(),
            })
            .await
            .ok();

        result
    }
}
