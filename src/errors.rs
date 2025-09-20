use std::borrow::Cow;

use actix_web::ResponseError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Not found")]
    InsufficientFunds,
    #[error("Not found")]
    NotFound,
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::Io(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Error::Custom(_) => actix_web::http::StatusCode::IM_A_TEAPOT,
            Error::Db(e) => match e {
                sqlx::Error::RowNotFound => actix_web::http::StatusCode::NOT_FOUND,
                sqlx::Error::Database(e) if e.code() == Some(Cow::Borrowed("23505")) => {
                    actix_web::http::StatusCode::CONFLICT
                }
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::InsufficientFunds => actix_web::http::StatusCode::BAD_REQUEST,
            Error::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            Error::RateLimitExceeded => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let body = json!({"error": self.to_string()}).to_string();
        actix_web::HttpResponse::build(self.status_code()).body(body)
    }
}
