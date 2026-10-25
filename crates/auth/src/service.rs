use crate::{
    AuditAction, AuditEvent, AuditOutcome, AuditSink, AuthError, Clock, LoginThrottle,
    PasswordHasher, SessionRepository, TokenCodec, UserRepository,
};
use chrono::Duration;

pub struct AuthService<U, S, C, P, T, L, A> {
    users: U,
    sessions: S,
    clock: C,
    password_hasher: P,
    token_codec: T,
    throttle: L,
    audit: A,
    session_ttl: Duration,
}

impl<U, S, C, P, T, L, A> AuthService<U, S, C, P, T, L, A>
where
    U: UserRepository,
    S: SessionRepository,
    C: Clock,
    P: PasswordHasher,
    T: TokenCodec,
    L: LoginThrottle,
    A: AuditSink,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        users: U,
        sessions: S,
        clock: C,
        password_hasher: P,
        token_codec: T,
        throttle: L,
        audit: A,
        session_ttl: Duration,
    ) -> Self {
        Self {
            users,
            sessions,
            clock,
            password_hasher,
            token_codec,
            throttle,
            audit,
            session_ttl,
        }
    }

    fn normalize_email(email: &str) -> String {
        email.trim().to_lowercase()
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<crate::User, AuthError> {
        if password.len() < 12 || password.len() > 128 {
            return Err(AuthError::WeakPassword);
        }
        let email = Self::normalize_email(email);
        let password_hash = self.password_hasher.hash(password).await?;
        let user = self
            .users
            .create(crate::NewUser {
                email,
                password_hash,
            })
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?;
        Ok(user)
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(crate::User, String), AuthError> {
        let email = Self::normalize_email(email);
        let now = self.clock.now();
        self.throttle.check_and_record(&email, now).await?;

        let user = self
            .users
            .find_by_email(&email)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?
            .ok_or(AuthError::Repository("invalid credentials".to_string()))?;

        let valid = self
            .password_hasher
            .verify(password, &user.password_hash)
            .await?;
        if !valid {
            let _ = self
                .audit
                .record(AuditEvent {
                    tenant_id: None,
                    actor_id: Some(user.id),
                    action: AuditAction::LoginFailed,
                    outcome: AuditOutcome::Failure,
                    occurred_at: now,
                })
                .await;
            return Err(AuthError::Repository("invalid credentials".to_string()));
        }

        let (token, token_hash) = self.token_codec.generate();
        let expires_at = now + self.session_ttl;
        let _session = self
            .sessions
            .create(crate::NewSession {
                user_id: user.id,
                token_hash,
                expires_at,
            })
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?;

        let _ = self
            .audit
            .record(AuditEvent {
                tenant_id: None,
                actor_id: Some(user.id),
                action: AuditAction::LoginSucceeded,
                outcome: AuditOutcome::Success,
                occurred_at: now,
            })
            .await;

        self.throttle.clear(&email).await;
        Ok((user, token))
    }

    pub async fn logout(&self, token: &str) -> Result<(), AuthError> {
        let hash = self.token_codec.hash(token);
        let session = self
            .sessions
            .find_by_token_hash(&hash)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?
            .ok_or(AuthError::SessionRevoked)?;
        self.sessions
            .revoke(session.id)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?;
        let _ = self
            .audit
            .record(AuditEvent {
                tenant_id: None,
                actor_id: Some(session.user_id),
                action: AuditAction::Logout,
                outcome: AuditOutcome::Success,
                occurred_at: self.clock.now(),
            })
            .await;
        Ok(())
    }

    pub async fn authenticate(&self, token: &str) -> Result<crate::User, AuthError> {
        let hash = self.token_codec.hash(token);
        let session = self
            .sessions
            .find_by_token_hash(&hash)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?
            .ok_or(AuthError::SessionRevoked)?;
        if session.revoked_at.is_some() {
            return Err(AuthError::SessionRevoked);
        }
        if session.expires_at < self.clock.now() {
            return Err(AuthError::SessionExpired);
        }
        let user = self
            .users
            .find_by_id(session.user_id)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?
            .ok_or(AuthError::Repository("user not found".to_string()))?;
        Ok(user)
    }
}
