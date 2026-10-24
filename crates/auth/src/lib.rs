//! Authentication domain boundary. Persistent sessions and HTTP handlers are
//! intentionally not implemented yet. See ADR/roadmap before selecting an auth
//! implementation; local authentication is the baseline and OAuth is optional.
use platform_core::UserId;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("account is disabled")]
    Disabled,
    #[error("authentication service is unavailable")]
    Unavailable,
}
