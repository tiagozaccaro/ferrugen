use chrono::{DateTime, Utc};
use platform_core::{TenantId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub tenant_id: Option<TenantId>,
    pub actor_id: Option<UserId>,
    pub action: String,
    pub outcome: AuditOutcome,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditOutcome {
    Success,
    Denied,
    Failure,
}

// Secret values, credentials, tokens and full request bodies must never be
// embedded in audit metadata.
