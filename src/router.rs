use std::sync::Arc;

use crate::actor_webhook_service_impl::actor::WebhookActor;
use crate::actor_webhook_service_impl::webhook_manager::ActorWebhookManager;
use crate::sqlx_db_impl::SqlxDbStore;
use crate::{db::DbStore, validator};
use actix::Actor;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;

pub async fn start_server(address: &str, database_url: &str) -> crate::errors::Result<()> {
    println!("Starting server at http://{}", address);
    let db_store = web::Data::from(
        Arc::new(SqlxDbStore::new_from_database_url(database_url)) as Arc<dyn DbStore>
    );
    let webhook_mgr = web::Data::from(Arc::new(ActorWebhookManager::new(
        db_store.clone().into_inner(),
    )) as Arc<dyn crate::webhook_service::WebhookManager>);
    let auth = HttpAuthentication::with_fn(validator::validator);
    let webhook_actor = WebhookActor::new(db_store.clone().into_inner());
    webhook_actor.start();
    HttpServer::new(move || {
        App::new()
            .app_data(db_store.clone())
            .app_data(webhook_mgr.clone())
            .wrap(auth.clone())
            .configure(config)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(crate::controller::index);
    cfg.service(crate::controller::account::create_account);
    cfg.service(crate::controller::account::get_account);
    cfg.service(crate::controller::transaction::transfer_funds);
    cfg.service(crate::controller::transaction::credit_account);
    cfg.service(crate::controller::transaction::debit_account);
    cfg.service(crate::controller::webhook::add_webhook);
}
