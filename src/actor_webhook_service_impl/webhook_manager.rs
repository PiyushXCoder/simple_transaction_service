use std::sync::Arc;

use crate::db::DbStore;
use crate::errors;
use crate::errors::Result;
use crate::webhook_service::webhook_manager::WebhookManager;

pub struct ActorWebhookManager {
    db_store: Arc<dyn DbStore>,
}

impl ActorWebhookManager {
    pub fn new(db_store: Arc<dyn DbStore>) -> Self {
        Self { db_store }
    }
}

#[async_trait::async_trait]
impl WebhookManager for ActorWebhookManager {
    async fn queue_webhook(
        &self,
        listening_account: &str,
        transaction_id: i32,
        event: &str,
        message: &str,
    ) -> errors::Result<()> {
        Ok(())
    }

    async fn poll(&self) -> Result<()> {
        Ok(())
    }
}
