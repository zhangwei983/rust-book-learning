use tokio::spawn;

pub async fn test() {
    println!("--- Start module: {}", module_path!());

    // Spawn a new task(an asynchronous green thread).
    let handle = spawn(async {
        println!("Hello from a spawned task.");
        "returned value"
    });

    println!("Spawned task is running in the background.");

    let result = handle.await.unwrap();
    println!("Spawned task returned: {:?}", result);

    println!("--- End module: {}", module_path!());
}
