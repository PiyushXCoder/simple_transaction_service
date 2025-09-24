use sqlx::types::time::PrimitiveDateTime;

use crate::errors::Error;

#[async_trait::async_trait]
pub trait Idempotency {
    async fn get_idempotency_item(&mut self, id: &str) -> Result<Option<IdempotencyItem>, Error>;
    async fn set_idempotency_item(
        &mut self,
        key: &str,
        response: Vec<u8>,
        status_code: i32,
    ) -> Result<(), Error>;
}

pub struct IdempotencyItem {
    pub key: String,
    pub response: Vec<u8>,
    pub status_code: i32,
    pub created_at: PrimitiveDateTime,
}
