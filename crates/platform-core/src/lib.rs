use thiserror::Error;

mod config;
mod ids;
pub use config::AppConfig;
pub use ids::{SessionId, TenantId, UserId};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("operation is not authorized")]
    Unauthorized,
    #[error("resource was not found")]
    NotFound,
}
