use sqlx::types::time::PrimitiveDateTime;

use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Transaction {
    async fn create_transaction(
        &self,
        sender: &Username,
        receiver: &Username,
        amount: i64,
    ) -> Result<(), Error>;
    async fn get_transaction(&self, id: i32) -> Result<Option<TransactionInfo>, Error>;
    async fn list_transactions(&self) -> Result<Vec<TransactionInfo>, Error>;
}

pub struct TransactionInfo {
    pub id: i32,
    pub sender: Username,
    pub receiver: Username,
    pub amount: i64,
    pub timestamp: PrimitiveDateTime,
}
