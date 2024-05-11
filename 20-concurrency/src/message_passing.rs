use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn test_single_message(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("Val is {}", val); // Compile error as the ownership of val transferred to the main thread.
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn test_multiple_messages(){
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // Clone tx to have multiple transmitters.

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn test_message_passing() {
    println!("--- Start module: {}", module_path!());

    test_single_message();
    test_multiple_messages();

    println!("--- End module: {}", module_path!());
}
