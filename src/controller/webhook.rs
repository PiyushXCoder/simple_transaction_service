use crate::{core::webhook, db::RefTransaction, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, post, web};

#[post("/add_webhook")]
async fn add_webhook(
    eq_body: web::Json<AddWebhookRequest>,
    db_transaction: web::ReqData<RefTransaction>,
) -> ActixResult<impl Responder> {
    let resp = webhook::add_webhook(eq_body.into_inner(), db_transaction.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}
