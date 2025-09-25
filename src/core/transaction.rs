use std::sync::Arc;

use crate::db::RefTransaction;
use crate::webhook_service::WebhookManager;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn transfer_funds(
    req: TransferFundsRequest,
    db_transaction: RefTransaction,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_transaction
        .lock()
        .await
        .create_transaction(&req.sender, &req.receiver, req.amount)
        .await?;

    webhook_mgr
        .queue_webhook(
            db_transaction.clone(),
            &req.sender,
            id,
            "Transfer Funds",
            format!("Transferred {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;

    webhook_mgr
        .queue_webhook(
            db_transaction.clone(),
            &req.receiver,
            id,
            "Transfer Funds",
            format!(
                "Transferred of {} from {} to {}",
                req.amount, req.sender, req.receiver
            )
            .as_str(),
        )
        .await?;

    Ok(TransactionResponse { id })
}

pub async fn credit_account(
    req: CreditAccountRequest,
    db_transaction: RefTransaction,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_transaction
        .lock()
        .await
        .credit_account(&req.receiver, req.amount)
        .await?;
    webhook_mgr
        .queue_webhook(
            db_transaction.clone(),
            &req.receiver,
            id,
            "Credit Funds",
            format!("Credited {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;
    Ok(TransactionResponse { id })
}

pub async fn debit_account(
    req: DebitAccountRequest,
    db_transaction: RefTransaction,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_transaction
        .lock()
        .await
        .debit_account(&req.receiver, req.amount)
        .await?;
    webhook_mgr
        .queue_webhook(
            db_transaction.clone(),
            &req.receiver,
            id,
            "Debit Funds",
            format!("Debited {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;
    Ok(TransactionResponse { id })
}
