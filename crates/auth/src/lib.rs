use thiserror::Error;

pub mod password;
pub mod ports;
pub mod service;
pub mod throttle;
pub mod token;

pub use password::Argon2PasswordHasher;
pub use service::AuthService;
pub use throttle::InMemoryLoginThrottle;
pub use token::OpaqueTokenCodec;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("email already taken")]
    EmailTaken,
    #[error("password must be 12-128 characters")]
    WeakPassword,
    #[error("too many attempts")]
    TooManyAttempts,
    #[error("session expired")]
    SessionExpired,
    #[error("session revoked")]
    SessionRevoked,
    #[error("repository error: {0}")]
    Repository(String),
    #[error("hashing error: {0}")]
    Hashing(String),
}

pub use platform_core::SessionId;
pub use ports::*;

pub use application::audit::{
    AuditAction, AuditError as AuditErrorType, AuditEvent, AuditOutcome, AuditSink,
};
