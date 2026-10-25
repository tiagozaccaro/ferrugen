use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque identifier for a user. Convert external input only after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

/// Opaque identifier for a tenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub Uuid);

/// Opaque identifier for a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}
impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}
impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}
