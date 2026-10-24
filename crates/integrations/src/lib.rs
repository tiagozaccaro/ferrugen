use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct IntegrationStatus {
    pub provider: String,
    pub configured: bool,
    pub healthy: Option<bool>,
}

#[async_trait]
pub trait Integration: Send + Sync {
    fn id(&self) -> &'static str;
    async fn health(&self) -> IntegrationStatus;
}

/// Provider credentials belong in a secret store or environment, never in
/// logs. All integrations are optional and disabled unless configured.
