use crate::{db::account::Username, errors::Error};

#[async_trait::async_trait]
pub trait Webhook {
    async fn create_webhook(&self, listening_account: &Username, url: &str) -> Result<(), Error>;
    async fn list_webhooks(&self, listening_account: &Username) -> Result<Vec<WebhookInfo>, Error>;
    async fn delete_webhook(&self, id: i32) -> Result<(), Error>;
}

pub struct WebhookInfo {
    pub id: i32,
    pub listening_account: Username,
    pub url: String,
}
