use super::Transaction;
use crate::errors::Error;

#[async_trait::async_trait]
pub trait DatabaseTranscation {
    async fn start_transaction(&self) -> Result<Box<dyn Transaction>, Error>;
}
