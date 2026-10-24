use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCursor {
    pub updated_at: DateTime<Utc>,
    pub opaque_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictPolicy { ServerWins, ClientWins, ManualResolution }

/// Never silently overwrite concurrent edits. Conflict behavior must be
/// specified per resource before sync is enabled.
