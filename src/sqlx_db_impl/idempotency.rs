use super::SqlxTransaction;
use crate::db::idempotency::{Idempotency, IdempotencyItem};

#[async_trait::async_trait]
impl Idempotency for SqlxTransaction {
    async fn get_idempotency_item(
        &mut self,
        id: &str,
    ) -> Result<Option<IdempotencyItem>, crate::errors::Error> {
        let rec = sqlx::query_as!(
            IdempotencyItem,
            r#"
            SELECT key, response, status_code, created_at
            FROM idempotency
            WHERE key = $1
            "#,
            id
        )
        .fetch_optional(&mut *self.tx)
        .await?;

        Ok(rec)
    }

    async fn set_idempotency_item(
        &mut self,
        key: &str,
        response: Vec<u8>,
        status_code: i32,
    ) -> Result<(), crate::errors::Error> {
        sqlx::query!(
            r#"
            INSERT INTO idempotency (key, response, status_code)
            VALUES ($1, $2, $3)
            ON CONFLICT (key) DO NOTHING
            "#,
            key,
            response,
            status_code as i32
        )
        .execute(&mut *self.tx)
        .await?;
        Ok(())
    }
}
