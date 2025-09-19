use super::SqlxDbStore;
use crate::db::account::{Account, Username};

#[async_trait::async_trait]
impl Account for SqlxDbStore {
    async fn create_account(
        &self,
        username: &Username,
        name: &str,
    ) -> Result<(), crate::errors::Error> {
        let query = sqlx::query!(
            r#"
            INSERT INTO account (username, name)
            VALUES ($1, $2)
            "#,
            username.as_str(),
            name
        );

        query.execute(&self.pg_pool).await?;
        Ok(())
    }

    async fn get_account(
        &self,
        username: &Username,
    ) -> Result<Option<crate::db::account::AccountInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::account::AccountInfo,
            r#"
            SELECT username, name, balance
            FROM account
            WHERE username = $1
            "#,
            username.as_str()
        );
        let account = query.fetch_optional(&self.pg_pool).await?;
        Ok(account)
    }

    async fn list_accounts(
        &self,
    ) -> Result<Vec<crate::db::account::AccountInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::account::AccountInfo,
            r#"
            SELECT username, name, balance
            FROM account
            ORDER BY username
            "#
        );
        let accounts = query.fetch_all(&self.pg_pool).await?;
        Ok(accounts)
    }
}
