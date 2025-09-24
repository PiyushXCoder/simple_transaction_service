use crate::db::RefTransaction;
use crate::{
    errors,
    messages::{requests::*, responses::*},
};

pub async fn add_webhook(
    req: AddWebhookRequest,
    db_transaction: RefTransaction,
) -> errors::Result<ResponseMessage> {
    db_transaction
        .lock()
        .await
        .add_webhook(&req.listening_account, &req.url)
        .await?;
    Ok(ResponseMessage {
        message: "Added webhook".to_string(),
    })
}
