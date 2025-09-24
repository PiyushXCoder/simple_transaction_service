use super::SqlxTransaction;
use crate::db::{account::Username, webhook::Webhook};

#[async_trait::async_trait]
impl Webhook for SqlxTransaction {
    async fn add_webhook(
        &mut self,
        username: &Username,
        url: &str,
    ) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            INSERT INTO webhook (listening_account, url)
            VALUES ($1, $2)
            "#,
            username.as_str(),
            url
        );

        query.execute(&mut *self.tx).await?;
        Ok(())
    }

    async fn list_webhooks(
        &mut self,
        listening_account: &Username,
    ) -> Result<Vec<crate::db::webhook::WebhookInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::webhook::WebhookInfo,
            r#"
            SELECT id, listening_account, url
            FROM webhook
            WHERE listening_account = $1
            ORDER BY id
            "#,
            listening_account.as_str()
        );
        let webhooks = query.fetch_all(&mut *self.tx).await?;
        Ok(webhooks)
    }

    async fn delete_webhook(&mut self, webhook_id: i32) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            DELETE FROM webhook
            WHERE id = $1 
            "#,
            webhook_id,
        );

        let result = query.execute(&mut *self.tx).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::Error::NotFound);
        }
        Ok(())
    }

    async fn queue_webhook(
        &mut self,
        url: &str,
        listening_account: &Username,
        transaction_id: i32,
        event: &str,
        message: &str,
    ) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            INSERT INTO webhookqueue (url, listening_account, transaction_id, event, message)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            url,
            listening_account.as_str(),
            transaction_id,
            event,
            message
        );

        query.execute(&mut *self.tx).await?;
        Ok(())
    }

    async fn poll_webhook_queue(
        &mut self,
    ) -> Result<Vec<crate::db::webhook::QueuedWebhookItem>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::webhook::QueuedWebhookItem,
            r#" 
            SELECT id, url, listening_account, transaction_id, event, message, status
            FROM webhookqueue
            WHERE status = 'pending'
            ORDER BY id
            LIMIT 10
            "#
        );

        let items = query.fetch_all(&mut *self.tx).await?;
        Ok(items)
    }

    async fn mark_webhook_queue_item_as_sent(
        &mut self,
        id: i32,
    ) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            UPDATE webhookqueue
            SET status = 'sent'
            WHERE id = $1
            "#,
            id
        );

        let result = query.execute(&mut *self.tx).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::Error::NotFound);
        }
        Ok(())
    }
}
