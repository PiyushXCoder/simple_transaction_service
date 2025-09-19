use super::SqlxDbStore;
use crate::db::api_keys::ApiKeys;

#[async_trait::async_trait]
impl ApiKeys for SqlxDbStore {
    async fn create_api_key(&self) -> Result<String, crate::errors::Error> {
        let api_key = uuid::Uuid::new_v4().to_string();
        let query = sqlx::query!(
            r#"
            INSERT INTO api_keys (key)
            VALUES ($1)
            "#,
            api_key
        );
        query.execute(&self.pg_pool).await?;
        Ok(api_key)
    }

    async fn validate_api_key(&self, api_key: &str) -> Result<String, crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            SELECT key FROM api_keys WHERE key = $1
            "#,
            api_key
        );
        let record = query.fetch_one(&self.pg_pool).await?;
        Ok(record.key)
    }

    async fn list_api_keys(&self) -> Result<Vec<String>, crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            SELECT key FROM api_keys
            "#
        );
        let records = query.fetch_all(&self.pg_pool).await?;
        Ok(records.into_iter().map(|r| r.key).collect())
    }
}
