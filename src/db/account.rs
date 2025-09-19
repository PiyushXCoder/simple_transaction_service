use crate::errors::Error;

#[async_trait::async_trait]
pub trait Account {
    async fn create_account(&self, username: &Username, name: &str) -> Result<(), Error>;
    async fn get_account(&self, username: &Username) -> Result<Option<AccountInfo>, Error>;
    async fn list_accounts(&self) -> Result<Vec<AccountInfo>, Error>;
}

pub type Username = String;

pub struct AccountInfo {
    pub username: Username,
    pub name: String,
    pub balance: i64,
}
