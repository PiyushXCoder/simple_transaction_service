use super::{SqlxDbStore, SqlxTransaction};
use crate::db::{Transaction, database_transaction::DatabaseTranscation};

#[async_trait::async_trait]
impl DatabaseTranscation for SqlxDbStore {
    async fn start_transaction(&self) -> Result<Box<dyn Transaction>, crate::errors::Error> {
        let tx = SqlxTransaction::new(self.pg_pool.begin().await?);
        Ok(Box::new(tx))
    }
}

impl SqlxTransaction {
    pub fn new(tx: sqlx::Transaction<'static, sqlx::Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl Transaction for SqlxTransaction {
    async fn commit(self: Box<Self>) -> Result<(), crate::errors::Error> {
        self.tx.commit().await?;
        Ok(())
    }
}
