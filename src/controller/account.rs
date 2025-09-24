use crate::{core::account, db::RefTransaction, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, get, post, web};

#[post("/create_account")]
async fn create_account(
    eq_body: web::Json<CreateAccountRequest>,
    db_transaction: web::ReqData<RefTransaction>,
) -> ActixResult<impl Responder> {
    let resp = account::create_account(eq_body.into_inner(), db_transaction.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/get_account")]
async fn get_account(
    eq_body: web::Query<GetAccountRequest>,
    db_transaction: web::ReqData<RefTransaction>,
) -> ActixResult<impl Responder> {
    let resp = account::get_account(eq_body.into_inner(), db_transaction.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}
