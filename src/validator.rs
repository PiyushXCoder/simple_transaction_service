use crate::db::RefTransaction;
use actix_web::{Error, HttpMessage, dev::ServiceRequest, error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("no bearer header"), req));
    };
    let db_transaction = req.extensions().get::<RefTransaction>().cloned();
    let api_key = credentials.token();
    if let Some(db) = db_transaction {
        let mut db = db.lock().await;
        if let Err(_) = db.validate_api_key(api_key).await {
            return Err((error::ErrorUnauthorized("invalid api key"), req));
        }
    } else {
        return Err((error::ErrorInternalServerError("no db store"), req));
    }
    Ok(req)
}
