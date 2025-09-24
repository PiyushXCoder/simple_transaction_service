use std::{
    future::{Ready, ready},
    rc::Rc,
    sync::Arc,
};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::{future::LocalBoxFuture, lock::Mutex};

use crate::db::{DbStore, RefTransaction};

#[derive(Clone)]
pub struct TransactionInjector {
    db_store: Arc<dyn DbStore>,
}

impl TransactionInjector {
    pub fn new(db_store: Arc<dyn DbStore>) -> Self {
        Self { db_store }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TransactionInjector
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TransactionInjectorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TransactionInjectorMiddleware {
            service: Rc::new(service),
            db_store: self.db_store.clone(),
        }))
    }
}

pub struct TransactionInjectorMiddleware<S> {
    service: Rc<S>,
    db_store: Arc<dyn DbStore>,
}

impl<S, B> Service<ServiceRequest> for TransactionInjectorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let db_store = self.db_store.clone();
        let req = Box::new(req);

        Box::pin(async move {
            let tx: RefTransaction = Arc::new(Mutex::new(db_store.start_transaction().await?));
            req.extensions_mut().insert(tx);
            let fut = srv.call(*req);
            let res = fut.await?;
            let http_req = res.request().clone();
            let tx = http_req.extensions_mut().remove::<RefTransaction>();
            match tx {
                Some(tx) => {
                    if let Ok(tx) = Arc::try_unwrap(tx) {
                        let tx = tx.into_inner();
                        tx.commit().await?;
                    } else {
                        log::warn!(
                            "TransactionInjector: Transaction still has multiple references, cannot commit"
                        );
                    }
                }
                None => {
                    log::warn!("TransactionInjector: No transaction found in request extensions");
                }
            }
            Ok(res)
        })
    }
}
