use crate::db::RefTransaction;
use crate::webhook_service::WebhookManager;
use crate::{core::transaction, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, post, web};

#[post("/transfer_funds")]
async fn transfer_funds(
    eq_body: web::Json<TransferFundsRequest>,
    db_transaction: web::ReqData<RefTransaction>,
    webhook_mgr: web::Data<dyn WebhookManager>,
) -> ActixResult<impl Responder> {
    let resp = transaction::transfer_funds(
        eq_body.into_inner(),
        db_transaction.into_inner(),
        webhook_mgr.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/credit_account")]
async fn credit_account(
    eq_body: web::Json<CreditAccountRequest>,
    db_transaction: web::ReqData<RefTransaction>,
    webhook_mgr: web::Data<dyn WebhookManager>,
) -> ActixResult<impl Responder> {
    let resp = transaction::credit_account(
        eq_body.into_inner(),
        db_transaction.into_inner(),
        webhook_mgr.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/debit_account")]
async fn debit_account(
    eq_body: web::Json<DebitAccountRequest>,
    db_transaction: web::ReqData<RefTransaction>,
    webhook_mgr: web::Data<dyn WebhookManager>,
) -> ActixResult<impl Responder> {
    let resp = transaction::debit_account(
        eq_body.into_inner(),
        db_transaction.into_inner(),
        webhook_mgr.into_inner(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(resp))
}
