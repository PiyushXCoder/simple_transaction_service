use sqlx::types::time::PrimitiveDateTime;

use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Transaction {
    async fn create_transaction(
        &self,
        sender: &Username,
        receiver: &Username,
        amount: i64,
    ) -> Result<i32, Error>;
    async fn get_transaction(&self, id: i32) -> Result<Option<TransactionInfo>, Error>;
    async fn list_transactions(&self) -> Result<Vec<TransactionInfo>, Error>;
    async fn credit_account(&self, receiver: &Username, amount: i64) -> Result<i32, Error>;
    async fn debit_account(&self, sender: &Username, amount: i64) -> Result<i32, Error>;
}

pub struct TransactionInfo {
    pub id: i32,
    pub sender: Option<Username>,
    pub receiver: Username,
    pub amount: i64,
    pub timestamp: PrimitiveDateTime,
}
