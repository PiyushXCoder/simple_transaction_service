pub mod account;
pub mod api_keys;
pub mod idempotency;
pub mod transaction;
pub mod webhook;

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait DbStore:
    Send
    + Sync
    + account::Account
    + transaction::Transaction
    + api_keys::ApiKeys
    + webhook::Webhook
    + idempotency::Idempotency
    + 'static
{
}
