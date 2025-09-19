pub mod account;
pub mod api_keys;
pub mod transaction;

trait DbStore: Send + Sync + account::Account + transaction::Transaction + 'static {}
