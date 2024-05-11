use std::cell::RefCell;
use std::thread;

thread_local! {
    static FOO: RefCell<u32> = RefCell::new(1); // Declare a thread local variable.
}

pub fn test_thread_local() {
    println!("--- Start module: {}", module_path!());

    FOO.with(|foo| {
        assert_eq!(*foo.borrow(), 1);
        *foo.borrow_mut() = 2;
    });

    // Each thread starts out with the initial value of 1.
    thread::spawn(move || {
        FOO.with(|foo| {
            assert_eq!(*foo.borrow(), 1);
            *foo.borrow_mut() = 3;
            println!("Spawned thread: {}", *foo.borrow());
        })
    }).join().unwrap();

    // Retain the original value of 2 despite the child thread
    FOO.with(|foo| {
        println!("Main thread: {}", *foo.borrow());
    });

    // Another way to access the value via `with_borrow`.
    FOO.with_borrow(|foo| {
        println!("Main thread: {}", *foo);
    });

    println!("--- End module: {}", module_path!());
}
