use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { db: Arc::new(pool) }
    }
}
