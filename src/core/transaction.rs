use std::sync::Arc;

use crate::db::DbStore;
use crate::webhook_service::WebhookManager;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn transfer_funds(
    req: TransferFundsRequest,
    db_store: Arc<dyn DbStore>,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_store
        .create_transaction(&req.sender, &req.receiver, req.amount)
        .await?;

    webhook_mgr
        .queue_webhook(
            &req.sender,
            id,
            "Transfer Funds",
            format!("Transferred {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;

    webhook_mgr
        .queue_webhook(
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

    webhook_mgr.poll().await?;
    Ok(TransactionResponse { id })
}

pub async fn credit_account(
    req: CreditAccountRequest,
    db_store: Arc<dyn DbStore>,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_store.credit_account(&req.receiver, req.amount).await?;
    webhook_mgr
        .queue_webhook(
            &req.receiver,
            id,
            "Credit Funds",
            format!("Credited {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;
    webhook_mgr.poll().await?;
    Ok(TransactionResponse { id })
}

pub async fn debit_account(
    req: DebitAccountRequest,
    db_store: Arc<dyn DbStore>,
    webhook_mgr: Arc<dyn WebhookManager>,
) -> errors::Result<TransactionResponse> {
    let id = db_store.debit_account(&req.receiver, req.amount).await?;
    webhook_mgr
        .queue_webhook(
            &req.receiver,
            id,
            "Debit Funds",
            format!("Debited {} to {}", req.amount, req.receiver).as_str(),
        )
        .await?;
    webhook_mgr.poll().await?;
    Ok(TransactionResponse { id })
}
