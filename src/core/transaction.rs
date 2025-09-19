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
