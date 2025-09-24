use sqlx::types::time::PrimitiveDateTime;

use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Transaction {
    async fn create_transaction(
        &mut self,
        sender: &Username,
        receiver: &Username,
        amount: i64,
    ) -> Result<i32, Error>;
    async fn get_transaction(&mut self, id: i32) -> Result<Option<TransactionInfo>, Error>;
    async fn list_transactions(&mut self) -> Result<Vec<TransactionInfo>, Error>;
    async fn credit_account(&mut self, receiver: &Username, amount: i64) -> Result<i32, Error>;
    async fn debit_account(&mut self, sender: &Username, amount: i64) -> Result<i32, Error>;
}

pub struct TransactionInfo {
    pub id: i32,
    pub sender: Option<Username>,
    pub receiver: Username,
    pub amount: i64,
    pub timestamp: PrimitiveDateTime,
}
