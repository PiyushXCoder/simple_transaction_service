use crate::errors::Error;

#[async_trait::async_trait]
pub trait ApiKeys {
    async fn create_api_key(&mut self) -> Result<String, Error>;
    async fn validate_api_key(&mut self, api_key: &str) -> Result<String, Error>;
    async fn list_api_keys(&mut self) -> Result<Vec<String>, Error>;
}
