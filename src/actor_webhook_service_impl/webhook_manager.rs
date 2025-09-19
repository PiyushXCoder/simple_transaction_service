use std::sync::Arc;

use actix_broker::{Broker, SystemBroker};

use crate::actor_webhook_service_impl::actor::WebhookActorMessage;
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
        let db = self
            .db_store
            .list_webhooks(&listening_account.to_string())
            .await?;

        for webhook in db {
            self.db_store
                .queue_webhook(
                    &webhook.url,
                    &listening_account.to_string(),
                    transaction_id,
                    event,
                    message,
                )
                .await?;
        }
        Ok(())
    }

    async fn poll(&self) -> Result<()> {
        Broker::<SystemBroker>::issue_async(WebhookActorMessage::Poll);
        Ok(())
    }
}
