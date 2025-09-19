use super::SqlxDbStore;
use crate::db::{account::Username, transaction::Transaction};

#[async_trait::async_trait]
impl Transaction for SqlxDbStore {
    async fn create_transaction(
        &self,
        from: &Username,
        to: &Username,
        amount: i64,
    ) -> Result<(), crate::errors::Error> {
        todo!()
    }

    async fn get_transaction(
        &self,
        id: i64,
    ) -> Result<Option<crate::db::transaction::TransactionInfo>, crate::errors::Error> {
        todo!()
    }

    async fn list_transactions(
        &self,
    ) -> Result<Vec<crate::db::transaction::TransactionInfo>, crate::errors::Error> {
        todo!()
    }
}
