pub mod account;
pub mod api_keys;
pub mod transaction;

pub struct SqlxDbStore {
    pg_pool: sqlx::PgPool,
}

impl SqlxDbStore {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }
}
