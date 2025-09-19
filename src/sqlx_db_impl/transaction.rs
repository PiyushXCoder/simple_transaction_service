use super::SqlxDbStore;
use crate::db::{account::Username, transaction::Transaction};

#[async_trait::async_trait]
impl Transaction for SqlxDbStore {
    async fn create_transaction(
        &self,
        sender: &Username,
        receiver: &Username,
        amount: i64,
    ) -> Result<i32, crate::errors::Error> {
        let mut tx = self.pg_pool.begin().await?;

        let lock_sender_account = sqlx::query!(
            "SELECT balance FROM account WHERE username = $1 FOR UPDATE",
            sender
        );
        let lock_receiver_account = sqlx::query!(
            "SELECT balance FROM account WHERE username = $1 FOR UPDATE",
            receiver
        );

        let _ = lock_receiver_account.fetch_one(&mut *tx).await?.balance;
        let balance_sender = lock_sender_account.fetch_one(&mut *tx).await?.balance;

        if balance_sender < amount {
            return Err(crate::errors::Error::InsufficientFunds);
        }

        sqlx::query!(
            "UPDATE account SET balance = balance - $1 WHERE username = $2",
            amount,
            sender
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "UPDATE account SET balance = balance + $1 WHERE username = $2",
            amount,
            receiver
        )
        .execute(&mut *tx)
        .await?;

        let transaction_id = sqlx::query_scalar!(
            "INSERT INTO transaction (sender, receiver, amount) VALUES ($1, $2, $3) RETURNING id",
            sender,
            receiver,
            amount
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(transaction_id)
    }

    async fn get_transaction(
        &self,
        id: i32,
    ) -> Result<Option<crate::db::transaction::TransactionInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::transaction::TransactionInfo,
            "SELECT id, sender, receiver, amount, timestamp FROM transaction WHERE id = $1",
            id
        );
        let result = query.fetch_optional(&self.pg_pool).await?;
        Ok(result)
    }

    async fn list_transactions(
        &self,
    ) -> Result<Vec<crate::db::transaction::TransactionInfo>, crate::errors::Error> {
        let query = sqlx::query_as!(
            crate::db::transaction::TransactionInfo,
            "SELECT id, sender, receiver, amount, timestamp FROM transaction ORDER BY timestamp DESC"
        );
        let results = query.fetch_all(&self.pg_pool).await?;
        Ok(results)
    }

    async fn credit_account(
        &self,
        account: &Username,
        amount: i64,
    ) -> Result<i32, crate::errors::Error> {
        let mut tx = self.pg_pool.begin().await?;

        sqlx::query!(
            "UPDATE account SET balance = balance + $1 WHERE username = $2",
            amount,
            account
        )
        .execute(&mut *tx)
        .await?;

        let transaction_id = sqlx::query_scalar!(
            "INSERT INTO transaction (receiver, amount) VALUES ($1, $2) RETURNING id",
            account,
            amount
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(transaction_id)
    }

    async fn debit_account(
        &self,
        account: &Username,
        amount: i64,
    ) -> Result<i32, crate::errors::Error> {
        let mut tx = self.pg_pool.begin().await?;

        let lock_account = sqlx::query!(
            "SELECT balance FROM account WHERE username = $1 FOR UPDATE",
            account
        );

        let balance = lock_account.fetch_one(&mut *tx).await?.balance;
        if balance < amount {
            return Err(crate::errors::Error::InsufficientFunds);
        }

        sqlx::query!(
            "UPDATE account SET balance = balance - $1 WHERE username = $2",
            amount,
            account
        )
        .execute(&mut *tx)
        .await?;

        let transaction_id = sqlx::query_scalar!(
            "INSERT INTO transaction (receiver, amount) VALUES ($1, $2) RETURNING id",
            account,
            -amount
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(transaction_id)
    }
}
