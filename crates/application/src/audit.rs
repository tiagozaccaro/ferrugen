use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAction {
    LoginSucceeded,
    LoginFailed,
    Logout,
    RoleChanged,
    MemberRemoved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Denied,
    Failure,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub tenant_id: Option<Uuid>,
    pub actor_id: Option<Uuid>,
    pub action: AuditAction,
    pub outcome: AuditOutcome,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("audit recording failed: {0}")]
    Recording(String),
}

#[async_trait::async_trait]
pub trait AuditSink: Send + Sync {
    async fn record(&self, event: AuditEvent) -> Result<(), AuditError>;
}
