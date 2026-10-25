use crate::AuthError;
use chrono::{DateTime, Utc};
use platform_core::SessionId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewSession {
    pub user_id: Uuid,
    pub token_hash: Vec<u8>,
    pub expires_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: NewUser) -> Result<User, AuthError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError>;
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: NewSession) -> Result<Session, AuthError>;
    async fn find_by_token_hash(&self, hash: &[u8]) -> Result<Option<Session>, AuthError>;
    async fn revoke(&self, id: SessionId) -> Result<(), AuthError>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, AuthError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError>;
}

#[async_trait::async_trait]
pub trait TokenCodec: Send + Sync {
    fn generate(&self) -> (String, Vec<u8>);
    fn hash(&self, token: &str) -> Vec<u8>;
}

#[async_trait::async_trait]
pub trait LoginThrottle: Send + Sync {
    async fn check_and_record(&self, email: &str, now: DateTime<Utc>) -> Result<(), AuthError>;
    async fn clear(&self, email: &str);
}
