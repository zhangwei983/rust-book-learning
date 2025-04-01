mod basic_usage;
mod file;
mod join;
mod select;
mod spawn;
mod tcp;

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
    basic_usage::test().await;
    println!("");
    join::test().await;
    println!("");
    spawn::test().await;
    println!("");
    select::test().await;
    println!("");
    file::test().await.unwrap();
    println!("");
    tcp::test().await.unwrap();
}
