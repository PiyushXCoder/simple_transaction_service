use futures_util::lock::Mutex;
use std::sync::Arc;

pub mod account;
pub mod api_keys;
pub mod database_transaction;
pub mod idempotency;
pub mod transaction;
pub mod webhook;

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait DbStore: Send + Sync + database_transaction::DatabaseTranscation + 'static {}

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait Transaction:
    Send
    + Sync
    + account::Account
    + transaction::Transaction
    + api_keys::ApiKeys
    + webhook::Webhook
    + idempotency::Idempotency
    + 'static
{
    async fn commit(self: Box<Self>) -> Result<(), crate::errors::Error>;
}

pub type RefTransaction = Arc<Mutex<Box<dyn Transaction>>>;
