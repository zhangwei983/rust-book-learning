An example to show concurrency in Rust.

> [!NOTE]  
> For simplicity’s sake, we’ll refer to many of the problems as concurrent rather than being more precise by saying concurrent and/or parallel. 

## Threads

An executed program’s code is run in a `process`. Within a program, you can also use `threads` to run independent parts `simultaneously`.

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This leads to problems, such as:
- Race conditions
- Deadlocks
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

### Creating a New Thread with `spawn`

To create a new thread:
- call the `thread::spawn` function,
- and pass it a `closure` containing the code we want to run in the new thread.

```rust
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});
```

### Using `JoinHandle`

Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running. We can fix this by using `JoinHandle`.

`JoinHandle` is the return type of `thread::spawn`. A JoinHandle is an owned value. When we call the `join` method on `JoinHandle`, it will wait for its thread to finish.

```rust
handle.join().unwrap();
```

In above example, calling `join` on the `handle` blocks the thread currently running until the thread represented by the handle terminates. 

### Using `move` Closures with Threads

Using the `move` keyword with closures, the closure will 
- take ownership of the values it uses from the environment
- thus transfer ownership of those values from one thread to another.

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
```

By telling Rust to `move` ownership of `v` to the spawned thread, we’re guaranteeing Rust that the current thread won’t use v anymore.

## Message-passing Concurrency

## Shared-state Concurrency

## The Sync and Send Traits

## Thread Local Variable
