use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountRequest {
    pub username: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFundsRequest {
    pub sender: String,
    pub receiver: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreditAccountRequest {
    pub receiver: String,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebitAccountRequest {
    pub receiver: String,
    pub amount: i64,
}
