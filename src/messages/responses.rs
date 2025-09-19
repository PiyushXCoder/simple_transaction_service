use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub username: String,
    pub name: String,
    pub balance: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub id: i32,
}
