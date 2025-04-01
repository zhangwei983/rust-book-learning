use tokio::time::{Duration, sleep};

pub async fn test() {
    println!("--- Start module: {}", module_path!());

    let (result1, result2) = tokio::join!(
        async {
            sleep(Duration::from_secs(2)).await;
            println!("Result 1");
            1
        },
        async {
            sleep(Duration::from_secs(1)).await;
            println!("Result 2");
            2
        }
    );

    println!("Results: {:?}, {:?}", result1, result2);

    println!("--- End module: {}", module_path!());
}
