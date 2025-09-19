use crate::{core::account, db::DbStore, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, post, web};

#[post("/create_account")]
async fn create_account(
    eq_body: web::Json<CreateAccountRequest>,
    db_store: web::Data<dyn DbStore>,
) -> ActixResult<impl Responder> {
    account::create_account(eq_body.into_inner(), db_store.into_inner()).await?;
    Ok(HttpResponse::Ok())
}
