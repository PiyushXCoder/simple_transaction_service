use super::SqlxDbStore;
use crate::db::{account::Username, webhook::Webhook};

#[async_trait::async_trait]
impl Webhook for SqlxDbStore {
    async fn create_webhook(
        &self,
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

        query.execute(&self.pg_pool).await?;
        Ok(())
    }

    async fn list_webhooks(
        &self,
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
        let webhooks = query.fetch_all(&self.pg_pool).await?;
        Ok(webhooks)
    }

    async fn delete_webhook(&self, webhook_id: i32) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            DELETE FROM webhook
            WHERE id = $1 
            "#,
            webhook_id,
        );

        let result = query.execute(&self.pg_pool).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::Error::NotFound);
        }
        Ok(())
    }
}
