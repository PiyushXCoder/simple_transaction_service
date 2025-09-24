use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Webhook {
    async fn add_webhook(&mut self, listening_account: &Username, url: &str) -> Result<(), Error>;
    async fn list_webhooks(
        &mut self,
        listening_account: &Username,
    ) -> Result<Vec<WebhookInfo>, Error>;
    async fn delete_webhook(&mut self, id: i32) -> Result<(), Error>;

    async fn queue_webhook(
        &mut self,
        url: &str,
        listening_account: &Username,
        transaction_id: i32,
        event: &str,
        message: &str,
    ) -> Result<(), Error>;

    async fn poll_webhook_queue(&mut self) -> Result<Vec<QueuedWebhookItem>, Error>;

    async fn mark_webhook_queue_item_as_sent(&mut self, id: i32) -> Result<(), Error>;
}

pub struct WebhookInfo {
    pub id: i32,
    pub listening_account: Username,
    pub url: String,
}

pub struct QueuedWebhookItem {
    pub id: i32,
    pub url: String,
    pub listening_account: Username,
    pub transaction_id: i32,
    pub event: String,
    pub message: String,
    pub status: String,
}
