use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobId(pub Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobEnvelope {
    pub id: JobId,
    pub kind: String,
    pub payload_json: String,
    pub idempotency_key: Option<String>,
}

/// This crate defines job contracts only. A production adapter must provide
/// durable persistence, leases, retry/backoff, dead-letter handling and metrics.
