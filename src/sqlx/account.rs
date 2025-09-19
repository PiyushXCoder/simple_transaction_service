use super::SqlxDbStore;
use crate::db::account::{Account, Username};

#[async_trait::async_trait]
impl Account for SqlxDbStore {
    async fn create_account(
        &self,
        username: &Username,
        name: &str,
    ) -> Result<(), crate::errors::Error> {
        todo!()
    }

    async fn get_account(
        &self,
        username: &Username,
    ) -> Result<Option<crate::db::account::AccountInfo>, crate::errors::Error> {
        todo!()
    }

    async fn list_accounts(
        &self,
    ) -> Result<Vec<crate::db::account::AccountInfo>, crate::errors::Error> {
        todo!()
    }
}
