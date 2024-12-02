use actix::prelude::*;
use dev::{MessageResponse, OneshotSender};

enum Messages {
    Ping,
    Pong,
}

impl Message for Messages {
    type Result = Responses;
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle(
        self,
        _ctx: &mut <A as Actor>::Context,
        tx: Option<OneshotSender<<M as Message>::Result>>,
    ) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is alive.");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Actor is stopped.")
    }
}

impl Handler<Messages> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong,
        }
    }
}

#[actix::main]
async fn main() {
    let addr = MyActor.start();

    let ping = addr.send(Messages::Ping).await;
    match ping {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received"),
            Responses::GotPong => println!("Pong received"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }

    let pong = addr.send(Messages::Pong).await;
    match pong {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received"),
            Responses::GotPong => println!("Pong received"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }
}
