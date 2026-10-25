use persistence::Pool;

pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
