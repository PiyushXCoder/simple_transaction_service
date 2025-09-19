use std::sync::Arc;

use crate::db::DbStore;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn create_account(
    req: CreateAccountRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<ResponseMessage> {
    db_store.create_account(&req.username, &req.name).await?;
    Ok(ResponseMessage {
        message: "Account created successfully".to_string(),
    })
}

pub async fn get_account(
    req: GetAccountRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<AccountResponse> {
    let account = db_store.get_account(&req.username).await?;
    let account = account.ok_or(errors::Error::NotFound)?;

    Ok(AccountResponse {
        username: account.username,
        name: account.name,
        balance: account.balance,
    })
}
