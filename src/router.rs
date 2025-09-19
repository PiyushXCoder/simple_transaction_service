use actix_web::{App, HttpServer, web};

pub async fn start_server(address: &str) -> crate::errors::Result<()> {
    println!("Starting server at http://{}", address);
    HttpServer::new(|| App::new().configure(config))
        .bind(address)?
        .run()
        .await?;
    Ok(())
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {}
