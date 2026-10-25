use auth::{
    AuthError, NewSession, NewUser, Session, SessionId, SessionRepository, User, UserRepository,
};
use persistence::{PgSessionRepository, PgUserRepository, SessionRow, UserRow};
use uuid::Uuid;

pub struct PgUserRepositoryAdapter(pub PgUserRepository);
pub struct PgSessionRepositoryAdapter(pub PgSessionRepository);

#[async_trait::async_trait]
impl UserRepository for PgUserRepositoryAdapter {
    async fn create(&self, user: NewUser) -> Result<User, AuthError> {
        let id = Uuid::new_v4();
        self.0
            .create(id, &user.email, None, &user.password_hash)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?;
        Ok(User {
            id,
            email: user.email,
            password_hash: user.password_hash,
        })
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        self.0
            .find_by_email(email)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))
            .map(|opt| {
                opt.map(|r: UserRow| User {
                    id: r.id,
                    email: r.email,
                    password_hash: r.password_hash,
                })
            })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        self.0
            .find_by_id(id)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))
            .map(|opt| {
                opt.map(|r: UserRow| User {
                    id: r.id,
                    email: r.email,
                    password_hash: r.password_hash,
                })
            })
    }
}

#[async_trait::async_trait]
impl SessionRepository for PgSessionRepositoryAdapter {
    async fn create(&self, session: NewSession) -> Result<Session, AuthError> {
        let id = SessionId::new();
        self.0
            .create(
                id.0,
                session.user_id,
                &session.token_hash,
                session.expires_at,
            )
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))?;
        Ok(Session {
            id,
            user_id: session.user_id,
            token_hash: session.token_hash,
            expires_at: session.expires_at,
            revoked_at: None,
        })
    }

    async fn find_by_token_hash(&self, hash: &[u8]) -> Result<Option<Session>, AuthError> {
        self.0
            .find_by_token_hash(hash)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))
            .map(|opt| {
                opt.map(|r: SessionRow| Session {
                    id: SessionId(r.id),
                    user_id: r.user_id,
                    token_hash: r.token_hash,
                    expires_at: r.expires_at,
                    revoked_at: r.revoked_at,
                })
            })
    }

    async fn revoke(&self, id: SessionId) -> Result<(), AuthError> {
        self.0
            .revoke(id.0)
            .await
            .map_err(|e| AuthError::Repository(e.to_string()))
    }
}
