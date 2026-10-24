use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub requested_capabilities: Vec<String>,
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("plugin manifest is invalid")]
    InvalidManifest,
    #[error("plugin requested a capability that was not granted")]
    CapabilityDenied,
}

/// Do not execute arbitrary third-party plugin code in-process without a
/// sandbox and a clear threat model. Start with declarative extensions.
