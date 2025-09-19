use std::sync::Arc;

use crate::db::DbStore;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn transfer_funds(
    req: TransferFundsRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<TransactionResponse> {
    let id = db_store
        .create_transaction(&req.sender, &req.receiver, req.amount)
        .await?;
    Ok(TransactionResponse { id })
}

pub async fn credit_account(
    req: CreditAccountRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<TransactionResponse> {
    let id = db_store.credit_account(&req.receiver, req.amount).await?;
    Ok(TransactionResponse { id })
}

pub async fn debit_account(
    req: DebitAccountRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<TransactionResponse> {
    let id = db_store.debit_account(&req.receiver, req.amount).await?;
    Ok(TransactionResponse { id })
}
