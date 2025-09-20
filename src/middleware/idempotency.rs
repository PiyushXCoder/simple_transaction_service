use std::{
    future::{Ready, ready},
    rc::Rc,
    sync::Arc,
};

use actix_web::{
    Error, HttpResponseBuilder,
    body::{self},
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::db::DbStore;

#[derive(Clone)]
pub struct Idempotency {
    db_store: Arc<dyn DbStore>,
}

impl Idempotency {
    pub fn new(db_store: Arc<dyn DbStore>) -> Self {
        Self { db_store }
    }
}

impl<S> Transform<S, ServiceRequest> for Idempotency
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = IdempotencyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IdempotencyMiddleware {
            service: Rc::new(service),
            db_store: self.db_store.clone(),
        }))
    }
}

pub struct IdempotencyMiddleware<S> {
    service: Rc<S>,
    db_store: Arc<dyn DbStore>,
}

impl<S> Service<ServiceRequest> for IdempotencyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
    // B: MessageBody + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let db_store = self.db_store.clone();

        Box::pin(async move {
            let idempotency_key = req
                .headers()
                .get("Idempotency-Key")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            if let Some(key) = &idempotency_key {
                let idempotency_item = db_store.get_idempotency_item(&key).await.unwrap();
                if let Some(item) = idempotency_item {
                    let body = item.response;
                    let status = item.status_code as u16;
                    let response = actix_web::HttpResponse::build(
                        actix_web::http::StatusCode::from_u16(status).unwrap(),
                    )
                    .body(body);
                    let res = req.into_response(response);
                    return Ok(res);
                }
            }

            let fut = srv.call(req);
            let res = fut.await?;
            let (res, bytes) = response_to_bytes(res).await;

            if let Some(key) = idempotency_key {
                let status_code = res.status().as_u16() as i32;
                db_store
                    .set_idempotency_item(&key, bytes, status_code)
                    .await
                    .unwrap();
            }
            Ok(res)
        })
    }
}

pub(crate) async fn response_to_bytes(res: ServiceResponse) -> (ServiceResponse, Vec<u8>) {
    let mut result = Vec::new();
    let status = res.status();
    let headers = res.headers().clone();

    let req = res.request().clone();
    let body = body::to_bytes(res.into_body()).await.unwrap();
    result.extend_from_slice(&body);

    let mut res_builder = HttpResponseBuilder::new(status);
    for h in headers {
        res_builder.insert_header(h);
    }
    let res = res_builder.body(body);

    (ServiceResponse::new(req, res), result)
}
