use std::{
    collections::{HashMap, VecDeque},
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
    counter: Arc<RwLock<HashMap<String, RwLock<VecDeque<Instant>>>>>,
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
    counter: Arc<RwLock<HashMap<String, RwLock<VecDeque<Instant>>>>>,
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
                let timestamp_of_visits = api_counter.get(&api_key);
                let current_time = Instant::now();
                match timestamp_of_visits {
                    Some(locked_timestamps) => {
                        let mut timestamps = locked_timestamps.write().unwrap();
                        timestamps.retain_mut(|ts| current_time.duration_since(*ts).as_secs() < 60);
                        if timestamps.len() as i32 >= MAX_REQUESTS_PER_MINUTE {
                            let response = actix_web::HttpResponse::from_error(
                                errors::Error::RateLimitExceeded,
                            );
                            let res = req.into_response(response);
                            return Ok(res);
                        } else {
                            timestamps.push_back(current_time);
                        }
                    }
                    None => {
                        drop(api_counter);
                        let mut api_counter = counter.write().unwrap();
                        api_counter.insert(api_key, RwLock::new(VecDeque::from([current_time])));
                    }
                }
            }

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}
