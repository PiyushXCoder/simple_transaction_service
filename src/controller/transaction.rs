use crate::messages::requests::*;
use actix_web::{HttpResponse, Responder, post, web};

#[post("/transfer_funds")]
async fn transfer_funds(eq_body: web::Json<TransferFundsRequest>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/credit_account")]
async fn credit_account(eq_body: web::Json<CreditAccountRequest>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/debit_account")]
async fn debit_account(eq_body: web::Json<DebitAccountRequest>) -> impl Responder {
    HttpResponse::Ok()
}
