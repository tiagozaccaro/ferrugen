//! Concrete adapters and runtime wiring live here. Keep domain/application
//! crates independent of specific databases, filesystems and providers.
pub const DEFAULT_STORAGE_BACKEND: &str = "local-filesystem";

pub mod wiring;
