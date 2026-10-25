#![allow(dead_code)]
use persistence::PgPool;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MigrationState {
    Applied,
    Failed,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub migration_state: Option<MigrationState>,
}
