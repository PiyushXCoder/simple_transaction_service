use crate::{db::DbStore, open_telemetry::new_span};
use actix::prelude::*;
use actix_broker::{BrokerSubscribe, SystemBroker};
use log::{error, info};
use opentelemetry::{global::ObjectSafeSpan, trace::SpanKind};
use std::sync::Arc;

pub struct WebhookActor {
    db_store: Arc<dyn DbStore>,
}

impl WebhookActor {
    pub fn new(db_store: Arc<dyn DbStore>) -> Self {
        Self { db_store }
    }
}

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub enum WebhookActorMessage {
    Poll,
}

impl Actor for WebhookActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let mut span = new_span("WebhookActor::started", SpanKind::Internal);
        span.set_status(opentelemetry::trace::Status::Ok);
        log::info!("WebhookActor started");
        self.subscribe_async::<SystemBroker, WebhookActorMessage>(ctx);
        span.end();
    }
}

impl Handler<WebhookActorMessage> for WebhookActor {
    type Result = ();
    fn handle(&mut self, _: WebhookActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        let mut span = new_span("WebhookActor::Poll", SpanKind::Internal);
        span.set_status(opentelemetry::trace::Status::Ok);

        let db_store = self.db_store.clone();
        actix::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await; // Throttle to avoid busy looping

            let mut tx = match db_store.start_transaction().await {
                Ok(tx) => tx,
                Err(e) => {
                    error!("Failed to start transaction: {}", e);
                    return;
                }
            };
            match tx.poll_webhook_queue().await {
                Ok(items) => {
                    for item in items {
                        let client = reqwest::Client::new();
                        let res = client
                            .post(&item.url)
                            .header("Content-Type", "application/json")
                            .body(item.message.clone())
                            .send()
                            .await;

                        match res {
                            Ok(response) => {
                                if response.status().is_success() {
                                    info!("Successfully sent webhook to {}", item.url);
                                    if let Err(e) =
                                        tx.mark_webhook_queue_item_as_sent(item.id).await
                                    {
                                        error!("Failed to mark webhook queue item as sent: {}", e);
                                    }
                                } else {
                                    error!(
                                        "Failed to send webhook to {}: HTTP {}",
                                        item.url,
                                        response.status()
                                    );
                                }
                            }
                            Err(e) => {
                                error!("Failed to send webhook to {}: {}", item.url, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to poll webhook queue: {}", e);
                }
            }
            if let Err(e) = tx.commit().await {
                error!("Failed to commit transaction: {}", e);
            }
        });
        span.end();
    }
}
