use actix_broker::{Broker, SystemBroker};

use crate::actor_webhook_service_impl::actor::WebhookActorMessage;
use crate::db::RefTransaction;
use crate::errors;
use crate::errors::Result;
use crate::webhook_service::webhook_manager::WebhookManager;

pub struct ActorWebhookManager;

#[async_trait::async_trait]
impl WebhookManager for ActorWebhookManager {
    async fn queue_webhook(
        &self,
        db_transaction: RefTransaction,
        listening_account: &str,
        transaction_id: i32,
        event: &str,
        message: &str,
    ) -> errors::Result<()> {
        let mut tx = db_transaction.lock().await;
        let db = tx.list_webhooks(&listening_account.to_string()).await?;

        for webhook in db {
            tx.queue_webhook(
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
