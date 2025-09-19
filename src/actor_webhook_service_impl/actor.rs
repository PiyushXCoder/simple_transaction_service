use crate::db::DbStore;
use actix::prelude::*;
use actix_broker::{BrokerSubscribe, SystemBroker};
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
        self.subscribe_async::<SystemBroker, WebhookActorMessage>(ctx);
    }
}

impl Handler<WebhookActorMessage> for WebhookActor {
    type Result = ();
    fn handle(&mut self, _: WebhookActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        let db_store = self.db_store.clone();
        actix::spawn(async move {
            match db_store.poll_webhook_queue().await {
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
                                    if let Err(e) =
                                        db_store.mark_webhook_queue_item_as_sent(item.id).await
                                    {
                                        println!(
                                            "Failed to mark webhook queue item as sent: {}",
                                            e
                                        );
                                    }
                                } else {
                                    println!(
                                        "Failed to send webhook to {}: HTTP {}",
                                        item.url,
                                        response.status()
                                    );
                                }
                            }
                            Err(e) => {
                                println!("Failed to send webhook to {}: {}", item.url, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to poll webhook queue: {}", e);
                }
            }
        });
    }
}
