This example shows how to use `async/.await` in Rust.

`async/.await` is Rust's built-in tool for 
- writing `asynchronous` functions that look like synchronous code.
- making it possible to yield control of the current thread rather than blocking, allowing other code to make progress while waiting on an operation to complete.

## async

To create an asynchronous function, you can use the `async` fn syntax:

```rust
async fn do_something() { /* ... */ }
```

The value returned by `async` fn is a `Future`. 

## .await

An `.await` expression is a syntactic construct for suspending a computation provided by an implementation of [std::future::IntoFuture](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) until the given future is ready to produce a value. 

Inside an `async` fn, you can use `.await` to wait for the completion of another type that implements the `Future` trait, such as the output of another `async` fn.

An `.await` expression is roughly equivalent to the following persuedo code:

```rust
match operand.into_future() {
    mut pinned => loop {
        let mut pin = unsafe { Pin::new_unchecked(&mut pinned) };
        match Pin::future::poll(Pin::borrow(&mut pin), &mut current_context) {
            Poll::Ready(r) => break r,
            Poll::Pending => yield Poll::Pending,
        }
    }
}
```

The `.await` keyword desugars into a call to `IntoFuture::into_future` first before polling the future to completion.

## block_on vs .await

- `block_on` blocks the current thread until the provided future has run to completion. 
- `.await` doesn't block the current thread, but instead asynchronously waits for the future to complete, allowing other tasks to run if the future is currently unable to make progress.

## join! and try_join!

`join!` is like `.await` but can wait for multiple futures concurrently.

For futures which return `Result`, consider using `try_join!` rather than `join!`.

- `join!` only completes once `all` subfutures have completed, it'll continue processing other futures even after one of its subfutures has returned an `Err`.
- `try_join!` will complete immediately if one of the subfutures returns an error.
