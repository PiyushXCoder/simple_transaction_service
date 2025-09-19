pub mod account;
pub mod transaction;
pub mod webhook;

#[actix_web::get("/")]
async fn index() -> &'static str {
    "Check README.md for usage instructions."
}
