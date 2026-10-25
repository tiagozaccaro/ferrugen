#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use audit::RecordingAuditSink;
use auth::{
    Argon2PasswordHasher, AuthError, AuthService, Clock, InMemoryLoginThrottle, OpaqueTokenCodec,
    SessionRepository, User, UserRepository,
};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

struct InMemUserRepo(Mutex<HashMap<String, User>>);
impl InMemUserRepo {
    fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}
#[async_trait::async_trait]
impl UserRepository for InMemUserRepo {
    async fn create(&self, user: auth::NewUser) -> Result<User, AuthError> {
        let mut m = self.0.lock().unwrap();
        if m.contains_key(&user.email) {
            return Err(AuthError::EmailTaken);
        }
        let u = User {
            id: Uuid::new_v4(),
            email: user.email.clone(),
            password_hash: user.password_hash,
        };
        m.insert(user.email, u.clone());
        Ok(u)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        Ok(self.0.lock().unwrap().get(email).cloned())
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .values()
            .find(|u| u.id == id)
            .cloned())
    }
}

struct InMemSessionRepo(Mutex<Vec<auth::Session>>);
impl InMemSessionRepo {
    fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }
}
#[async_trait::async_trait]
impl SessionRepository for InMemSessionRepo {
    async fn create(&self, s: auth::NewSession) -> Result<auth::Session, AuthError> {
        let sess = auth::Session {
            id: platform_core::SessionId::new(),
            user_id: s.user_id,
            token_hash: s.token_hash,
            expires_at: s.expires_at,
            revoked_at: None,
        };
        self.0.lock().unwrap().push(sess.clone());
        Ok(sess)
    }
    async fn find_by_token_hash(&self, hash: &[u8]) -> Result<Option<auth::Session>, AuthError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.token_hash == hash)
            .cloned())
    }
    async fn revoke(&self, id: platform_core::SessionId) -> Result<(), AuthError> {
        if let Some(s) = self.0.lock().unwrap().iter_mut().find(|s| s.id == id) {
            s.revoked_at = Some(Utc::now());
        }
        Ok(())
    }
}

struct FixedClock;
impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[tokio::test]
async fn register_and_login() {
    let users = InMemUserRepo::new();
    let sessions = InMemSessionRepo::new();
    let clock = FixedClock;
    let hasher = Argon2PasswordHasher;
    let codec = OpaqueTokenCodec;
    let throttle = InMemoryLoginThrottle::new(5, Duration::minutes(15), Duration::minutes(15));
    let audit = RecordingAuditSink::new();
    let svc = AuthService::new(
        users,
        sessions,
        clock,
        hasher,
        codec,
        throttle,
        audit,
        Duration::days(30),
    );

    let user = svc
        .register("a@example.com", "password123456")
        .await
        .unwrap();
    assert_eq!(user.email, "a@example.com");

    let (u, token) = svc.login("a@example.com", "password123456").await.unwrap();
    assert_eq!(u.id, user.id);
    assert!(!token.is_empty());

    let authed = svc.authenticate(&token).await.unwrap();
    assert_eq!(authed.id, user.id);
}
