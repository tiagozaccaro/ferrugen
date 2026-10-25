use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("database configuration error")]
    Configuration(String),
    #[error("database connection error")]
    Connection(String),
    #[error("database operation failed")]
    Database(String),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Configuration(_) => PersistenceError::Configuration(err.to_string()),
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
                PersistenceError::Connection(err.to_string())
            }
            _ => PersistenceError::Database(err.to_string()),
        }
    }
}
