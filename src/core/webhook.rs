use std::sync::Arc;

use crate::db::DbStore;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn add_webhook(
    req: AddWebhookRequest,
    db_store: Arc<dyn DbStore>,
) -> errors::Result<ResponseMessage> {
    db_store
        .add_webhook(&req.listening_account, &req.url)
        .await?;
    Ok(ResponseMessage {
        message: "Added webhook".to_string(),
    })
}
