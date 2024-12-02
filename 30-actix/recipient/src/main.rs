use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
struct OrderShipped {
    order_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Ship {
    order_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe {
    pub recipient: Recipient<OrderShipped>,
}

struct OrderEvents {
    subscribers: Vec<Recipient<OrderShipped>>,
}

impl OrderEvents {
    fn new() -> Self {
        OrderEvents {
            subscribers: vec![],
        }
    }
}

impl Actor for OrderEvents {
    type Context = Context<Self>;
}

impl OrderEvents {
    fn notify(&mut self, order_id: usize) {
        for sub in &self.subscribers {
            sub.do_send(OrderShipped { order_id });
        }
    }
}

// Send message to OrderEvents to subscribe.
impl Handler<Subscribe> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _ctx: &mut Self::Context) {
        println!("Recipient registered: {:?}", msg.recipient);
        self.subscribers.push(msg.recipient);
    }
}

impl Handler<Ship> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Ship, _ctx: &mut Self::Context) -> Self::Result {
        self.notify(msg.order_id);
        System::current().stop();
    }
}

struct EmailSubscriber;

impl Actor for EmailSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for EmailSubscriber {
    type Result = ();

    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("Email sent for order {}", msg.order_id)
    }
}

struct SmsSubscriber;

impl Actor for SmsSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for SmsSubscriber {
    type Result = ();

    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("SMS sent for order {}", msg.order_id)
    }
}

#[actix::main]
async fn main() -> Result<(), actix::MailboxError> {
    // Create recipients, types are deduced from Hanlders.
    let email_subscriber = Subscribe {
        recipient: EmailSubscriber {}.start().recipient(),
    };
    let sms_subscriber = Subscribe {
        recipient: SmsSubscriber {}.start().recipient(),
    };

    // Send SubScribe messages to register recipients to OrderEvents.
    let order_events_address = OrderEvents::new().start();
    order_events_address.send(email_subscriber).await?;
    order_events_address.send(sms_subscriber).await?;

    // Send Ship message to notify the recipient.
    order_events_address.send(Ship { order_id: 1 }).await?;

    Ok(())
}
