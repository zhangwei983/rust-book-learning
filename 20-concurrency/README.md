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

## Message-passing Between Threads

Message passing means threads or actors communicate by sending each other messages containing data.

A slogan from Go:

```
Do not communicate by sharing memory; instead, share memory by communicating.
```

Below is an example of using channel to pass message between threads.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

### Channel

A `channel`:

- is a general programming concept by which data is sent from one thread to another.
- has two halves: a `transmitter` and a `receiver`.

A channel is said to be `closed` if either the transmitter or receiver half is dropped.

The above example shows how to create a new channel using the `mpsc::channel` function. 

The `mpsc` stands for `multiple` producer, `single` consumer. The way Rust’s standard library implements channels means a channel can have `multiple` sending ends that produce values but only `one` receiving end that consumes those values.

Here a `let` statement is used to extract the `tx` and `rx` of the tuple returned by mpsc::channel.

### Sending messages

```rust
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});
```

This code snippet shows how to send message from a spawn thread.

[!NOTE]  
> The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.

### Receiving messages

```rust
tx.send(val).unwrap();
```

The receiver has two useful methods: 
- `recv` which will block the main thread’s execution and wait until a value is sent down the channel.
- and `try_recv`, which doesn’t block, but will instead return a `Result<T, E>` immediately.

```rust
for received in rx {
    println!("Got: {}", received);
}
```

We can treat rx as an `iterator` instead of calling `recv` function. When the channel is closed, iteration will end.

### Multiple Transmitters

We can use clone to create more transmitters, like below

```rust
let tx1 = tx.clone();
```

## Shared-state Concurrency

Shared memory concurrency is like `multiple` ownership: multiple threads can access the same memory location at the same time.

### Mutex

`Mutex` is an abbreviation for `mutual exclusion`, as in, a mutex allows `only one` thread to access some data at any given time.

The `lock` is a data structure that is part of the mutex that keeps track of who currently has `exclusive` access to the data.

Mutexes have a reputation for being difficult to use:
- You must attempt to acquire the lock before using the data.
- When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

```rust
let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap();
    *num = 6;
}
```

This example shows: 
- creating a `Mutex<T>` using the associated function `new`.
- using the `lock` method to acquire the lock to access the data inside the mutex.
- the call to `lock` returns a smart pointer called `MutexGuard`.
- so we can treat the return value as a `mutable reference` to the data inside.

[!NOTE]
`Mutex<T>` provides `interior mutability` as the `Cell` family does.

The `MutexGuard` smart pointer implements 
- `Deref` trait to point at our inner data.
- `Drop` trait that releases the lock automatically when a MutexGuard goes out of scope,

### Sharing a Mutex<T> Between Multiple Threads

As we discussed before, `Rc<T>` support multiple ownership by creating a reference counted value, but it can only be used in `single-threaded` scenarios.

So we use `Atomic Reference Counting` with `Arc<T>` in this concurrent situations. You need to know that `atomics` work like `primitive` types but are safe to share across threads.

Check [this sample](./src/shared_state.rs) for how to use `Arc<T>`.

## The Sync and Send Traits

Interestingly, the Rust language has very few concurrency features.  Almost every concurrency feature we’ve talked about so far has been part of the standard library, not the language.

However, two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.

### Send Trait

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. 

- Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`.
- Almost all primitive types are `Send`, aside from `raw` pointers, which we’ll discuss later.
- Any type `composed` entirely of `Send` types is `automatically` marked as `Send`.

### Sycn Trait

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.

Any type T is Sync if `&T` (an immutable reference to T) is `Send`, meaning the reference can be sent safely to another thread.


### Implementing Send and Sync Manually Is `Unsafe`.

As marker traits, they don’t even have any methods to implement. 

Manually implementing these traits involves implementing `unsafe` Rust code. 

## Thread Local Variable

The [std::thread::LocalKey](https://doc.rust-lang.org/std/thread/struct.LocalKey.html) is a thread local storage key which owns its contents. It is instantiated with the `thread_local!` macro and the primary method is the `with` method. And there are `helpers to make working with `Cell` types easier.

The `std::thread_local` macro wraps any number of `static` declarations and makes them `thread local`. 

Note that only `shared references` (&T) to the inner data may be obtained, so a type such as `Cell` or `RefCell` is typically used to allow `mutating` access.

Check the [example](./src/thread_local.rs) to understand how to use `LocalKey` and `thread_local!`.
