use std::sync::Arc;

use crate::sqlx_db_impl::SqlxDbStore;
use crate::{db::DbStore, validator};
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;

pub async fn start_server(address: &str, database_url: &str) -> crate::errors::Result<()> {
    println!("Starting server at http://{}", address);
    let db_store = web::Data::from(
        Arc::new(SqlxDbStore::new_from_database_url(database_url)) as Arc<dyn DbStore>
    );
    let auth = HttpAuthentication::with_fn(validator::validator);
    HttpServer::new(move || {
        App::new()
            .app_data(db_store.clone())
            .wrap(auth.clone())
            .configure(config)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}

#[actix_web::get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}
