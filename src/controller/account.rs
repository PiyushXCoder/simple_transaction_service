use crate::{core::account, db::DbStore, messages::requests::*};
use actix_web::{HttpResponse, Responder, Result as ActixResult, get, post, web};

#[post("/create_account")]
async fn create_account(
    eq_body: web::Json<CreateAccountRequest>,
    db_store: web::Data<dyn DbStore>,
) -> ActixResult<impl Responder> {
    let resp = account::create_account(eq_body.into_inner(), db_store.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/get_account")]
async fn get_account(
    eq_body: web::Query<GetAccountRequest>,
    db_store: web::Data<dyn DbStore>,
) -> ActixResult<impl Responder> {
    let resp = account::get_account(eq_body.into_inner(), db_store.into_inner()).await?;
    Ok(HttpResponse::Ok().json(resp))
}
