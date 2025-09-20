use crate::db::DbStore;

pub mod account;
pub mod api_keys;
pub mod idempotency;
pub mod transaction;
pub mod webhook;

#[derive(Clone)]
pub struct SqlxDbStore {
    pg_pool: sqlx::PgPool,
}

impl SqlxDbStore {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub fn new_from_database_url(database_url: &str) -> Self {
        let pg_pool = sqlx::PgPool::connect_lazy(database_url)
            .expect("Failed to create Postgres connection pool");
        Self { pg_pool }
    }
}

impl DbStore for SqlxDbStore {}
