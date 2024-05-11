use std::thread;
use std::time::Duration;

pub fn test_thread_spawn(){
    println!("--- Start module: {}", module_path!());

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: Number {}", i);
            thread::sleep(Duration::from_millis(1)); // Force a thread to stop its execution for a short duration.
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("Main thread: Number {}", i);
        thread::sleep(Duration::from_millis(1)); // Force a thread to stop its execution for a short duration.
    }

    handle.join().unwrap(); // Wait for the spawned thread to finish before finishing main thread.

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // Move ownership of v to the spawned thead.
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    println!("--- End module: {}", module_path!());
}
