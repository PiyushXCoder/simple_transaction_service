use actix::prelude::*;
use actix_broker::{BrokerSubscribe, SystemBroker};

pub struct WebhookActor;

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub enum WebhookActorMessage {
    CheckQueue,
}

impl Actor for WebhookActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<SystemBroker, WebhookActorMessage>(ctx);
    }
}

impl Handler<WebhookActorMessage> for WebhookActor {
    type Result = ();
    fn handle(&mut self, msg: WebhookActorMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("WebhookActor received message: {:?}", msg);
    }
}
