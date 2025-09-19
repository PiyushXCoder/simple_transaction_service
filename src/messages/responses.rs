use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub username: String,
    pub name: String,
    pub balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub id: i32,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: String,
}
