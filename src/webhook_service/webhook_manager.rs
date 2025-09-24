use crate::{db::RefTransaction, errors::Result};

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait WebhookManager: Send + Sync + 'static {
    async fn queue_webhook(
        &self,
        db_transaction: RefTransaction,
        listening_account: &str,
        transaction_id: i32,
        event: &str,
        message: &str,
    ) -> Result<()>;

    async fn poll(&self) -> Result<()>;
}
