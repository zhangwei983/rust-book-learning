use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

#[actix::main]
async fn main() {
    let addr = MyActor { count: 10 }.start();

    let res = addr.send(Ping(10)).await;

    println!("Result: {}", res.unwrap() == 20);

    System::current().stop();
}
