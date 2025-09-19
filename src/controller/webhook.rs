use crate::{core::webhook, db::DbStore, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, post, web};

#[post("/add_webhook")]
async fn add_webhook(
    eq_body: web::Json<AddWebhookRequest>,
    db_store: web::Data<dyn DbStore>,
) -> ActixResult<impl Responder> {
    let resp = webhook::add_webhook(eq_body.into_inner(), db_store.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}
