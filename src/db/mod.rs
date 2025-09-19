pub mod account;
pub mod api_keys;
pub mod transaction;

#[allow(dead_code)]
#[async_trait::async_trait]
trait DbStore: Send + Sync + account::Account + transaction::Transaction + 'static {}
