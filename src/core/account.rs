use crate::db::RefTransaction;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn create_account(
    req: CreateAccountRequest,
    db_transaction: RefTransaction,
) -> errors::Result<ResponseMessage> {
    db_transaction
        .lock()
        .await
        .create_account(&req.username, &req.name)
        .await?;
    Ok(ResponseMessage {
        message: "Account created successfully".to_string(),
    })
}

pub async fn get_account(
    req: GetAccountRequest,
    db_transaction: RefTransaction,
) -> errors::Result<AccountResponse> {
    let account = db_transaction
        .lock()
        .await
        .get_account(&req.username)
        .await?;
    let account = account.ok_or(errors::Error::NotFound)?;

    Ok(AccountResponse {
        username: account.username,
        name: account.name,
        balance: account.balance,
    })
}
