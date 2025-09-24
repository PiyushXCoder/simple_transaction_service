// use super::SqlxDbStore;
use super::SqlxTransaction;
use crate::db::account::{Account, Username};

#[async_trait::async_trait]
impl Account for SqlxTransaction {
    async fn create_account(
        &mut self,
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

        query.execute(&mut *self.tx).await?;
        Ok(())
    }

    async fn get_account(
        &mut self,
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
        let account = query.fetch_optional(&mut *self.tx).await?;
        Ok(account)
    }

    async fn list_accounts(
        &mut self,
    ) -> Result<Vec<crate::db::account::AccountInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::account::AccountInfo,
            r#"
            SELECT username, name, balance
            FROM account
            ORDER BY username
            "#
        );
        let accounts = query.fetch_all(&mut *self.tx).await?;
        Ok(accounts)
    }
}
