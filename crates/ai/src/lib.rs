use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
}

#[derive(Debug, Error)]
pub enum AiError {
    #[error("provider is not configured")]
    NotConfigured,
    #[error("provider request failed")]
    ProviderFailure,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, AiError>;
}

/// Local runtime adapters should be implemented first. Model-weight licensing
/// must be reviewed separately from the inference runtime's license.
