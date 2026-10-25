use argon2::{
    Argon2, PasswordHash, PasswordHasher as ArgonHasher, PasswordVerifier,
    password_hash::SaltString,
};
use rand::rngs::OsRng;

use crate::{AuthError, PasswordHasher};

pub struct Argon2PasswordHasher;

#[async_trait::async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: &str) -> Result<String, AuthError> {
        if password.len() < 12 || password.len() > 128 {
            return Err(AuthError::WeakPassword);
        }
        let password = password.to_string();
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            argon2
                .hash_password(password.as_bytes(), &salt)
                .map(|h| h.to_string())
                .map_err(|e| AuthError::Hashing(e.to_string()))
        })
        .await
        .map_err(|e| AuthError::Hashing(e.to_string()))?
    }

    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let password = password.to_string();
        let hash = hash.to_string();
        tokio::task::spawn_blocking(move || {
            let parsed = PasswordHash::new(&hash).map_err(|e| AuthError::Hashing(e.to_string()))?;
            Ok(Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok())
        })
        .await
        .map_err(|e| AuthError::Hashing(e.to_string()))?
    }
}
