use std::sync::{Arc, Mutex};
use std::thread;

pub fn test_shared_state() {
    println!("--- Start module: {}", module_path!());

    let counter = Arc::new(Mutex::new(0)); // Use Arc for tultiple threads.
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {:?}", *counter.lock().unwrap());

    println!("--- End module: {}", module_path!());
}
