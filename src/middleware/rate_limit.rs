use std::{
    collections::HashMap,
    future::{Ready, ready},
    rc::Rc,
    sync::{Arc, RwLock},
    time::Instant,
};

use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::errors;

const MAX_REQUESTS_PER_MINUTE: i32 = 100;

#[derive(Clone)]
pub struct RateLimit {
    counter: Arc<RwLock<HashMap<String, (i32, Instant)>>>,
}

impl RateLimit {
    pub fn new() -> Self {
        RateLimit {
            counter: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service: Rc::new(service),
            counter: self.counter.clone(),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: Rc<S>,
    counter: Arc<RwLock<HashMap<String, (i32, Instant)>>>,
}

impl<S> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let counter = self.counter.clone();

        Box::pin(async move {
            let credendtials = req
                .extract::<actix_web_httpauth::extractors::bearer::BearerAuth>()
                .await;

            if let Ok(credentials) = credendtials {
                let api_key = credentials.token().to_string();

                let api_counter = counter.read().unwrap();
                let count = api_counter.get(&api_key);
                match count {
                    Some((count, timestamp)) => {
                        if timestamp.elapsed().as_secs() < 60 {
                            if *count >= MAX_REQUESTS_PER_MINUTE {
                                let response = actix_web::HttpResponse::from_error(
                                    errors::Error::RateLimitExceeded,
                                );
                                let res = req.into_response(response);
                                return Ok(res);
                            } else {
                                drop(api_counter);
                                let mut api_counter = counter.write().unwrap();
                                let (count, _) = api_counter.get_mut(&api_key).unwrap();
                                *count += 1;
                            }
                        } else {
                            drop(api_counter);
                            let mut api_counter = counter.write().unwrap();
                            let (count, timestamp) = api_counter.get_mut(&api_key).unwrap();
                            *count = 1;
                            *timestamp = Instant::now();
                        }
                    }
                    None => {
                        drop(api_counter);
                        let mut api_counter = counter.write().unwrap();
                        api_counter.insert(api_key, (1, Instant::now()));
                    }
                }
            }

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}
