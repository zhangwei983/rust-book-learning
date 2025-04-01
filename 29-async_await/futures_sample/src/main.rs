mod basic_usage;
mod simple_future;
use futures::executor::block_on;

async fn async_main() {
    basic_usage::test().await;
    println!("");
    simple_future::test().await;
}

fn main() {
    block_on(async_main());
}
