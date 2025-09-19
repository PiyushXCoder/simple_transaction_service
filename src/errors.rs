use actix_web::ResponseError;

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
}

pub type Result<T> = std::result::Result<T, Error>;

impl ResponseError for Error {}
