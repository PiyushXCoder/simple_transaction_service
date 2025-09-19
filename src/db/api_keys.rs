use crate::errors::Error;

#[async_trait::async_trait]
pub trait ApiKeys {
    async fn create_api_key(&self) -> Result<String, Error>;
    async fn validate_api_key(&self, api_key: &str) -> Result<String, Error>;
}
