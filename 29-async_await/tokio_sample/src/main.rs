mod hello_world;

// The #[tokio::main] macro transforms the async fn main() into a synchronous fn main()
// that initializes a runtime instance and executes the async main function.
// fn main() {
//    let mut runtime = tokio::runtime::Runtime::new().unwrap();
//    runtime.block_on(async {
//        ...
//    })
// }
#[tokio::main]
async fn main() {
    hello_world::test().await;
    println!("");
}
