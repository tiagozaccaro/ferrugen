use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Opaque identifier for a user. Convert external input only after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

/// Opaque identifier for a tenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub Uuid);

impl UserId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
impl Default for UserId { fn default() -> Self { Self::new() } }
impl TenantId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
impl Default for TenantId { fn default() -> Self { Self::new() } }

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("operation is not authorized")]
    Unauthorized,
    #[error("resource was not found")]
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_ids_are_distinct() {
        assert_ne!(UserId::new(), UserId::new());
        assert_ne!(TenantId::new(), TenantId::new());
    }
}
