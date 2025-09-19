use crate::db::DbStore;
use actix_web::{Error, dev::ServiceRequest, error, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("no bearer header"), req));
    };
    let api_key = credentials.token();
    if let Some(db) = req.app_data::<web::Data<dyn DbStore>>() {
        if let Err(_) = db.validate_api_key(api_key).await {
            return Err((error::ErrorUnauthorized("invalid api key"), req));
        }
    } else {
        return Err((error::ErrorInternalServerError("no db store"), req));
    }
    Ok(req)
}
