use std::sync::Arc;

use crate::db::DbStore;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn create_account(
    req: CreateAccountRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<ResponseMessage> {
    db_store.create_account(&req.username, &req.name).await?;
    Ok(ResponseMessage {
        message: "Account created successfully".to_string(),
    })
}
