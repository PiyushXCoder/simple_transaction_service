use super::SqlxDbStore;
use crate::db::api_keys::ApiKeys;

#[async_trait::async_trait]
impl ApiKeys for SqlxDbStore {
    async fn create_api_key(&self) -> Result<String, crate::errors::Error> {
        todo!()
    }

    async fn validate_api_key(&self, api_key: &str) -> Result<String, crate::errors::Error> {
        todo!()
    }
}
