use contracts::{ErrorResponse, HealthResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("transport error")]
    Transport,
    #[error("server returned an error: {0}")]
    Server(ErrorResponse),
}

pub trait ApiTransport {
    fn health(&self) -> Result<HealthResponse, ApiClientError>;
}
