use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Transaction {
    async fn create_transaction(
        &self,
        from: &Username,
        to: &Username,
        amount: i64,
    ) -> Result<(), Error>;
    async fn get_transaction(&self, id: i64) -> Result<Option<TransactionInfo>, Error>;
    async fn list_transactions(&self) -> Result<Vec<TransactionInfo>, Error>;
}

pub struct TransactionInfo {
    pub id: i64,
    pub from: Username,
    pub to: Username,
    pub amount: i128,
    pub timestamp: chrono::NaiveDateTime,
}
