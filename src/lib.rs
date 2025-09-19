pub mod controller;
pub mod core;
pub mod db;
pub mod errors;
pub mod router;
pub mod sqlx;

pub use router::start_server;
