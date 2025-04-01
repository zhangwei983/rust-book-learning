use tokio::time::{Duration, sleep};

pub async fn test() {
    println!("--- Start module: {}", module_path!());

    // Wait on multiple async tasks and returns when a single task completes.
    tokio::select! {
        _ = sleep(Duration::from_secs(2)) => {
            println!("2 seconds elapsed");
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("1 second elapsed");
        }
    }

    println!("--- End module: {}", module_path!());
}
