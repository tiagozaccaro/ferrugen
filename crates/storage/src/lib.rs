use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct LocalStorageConfig { pub root: PathBuf }

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("storage operation failed")]
    OperationFailed,
    #[error("path is not allowed")]
    InvalidPath,
}

/// Path validation and safe file operations must be implemented before this
/// adapter is exposed to untrusted upload names.
