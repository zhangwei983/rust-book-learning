An example shows errror handlings in Rust.

## Unrecoverable Errors with panic!

By default, `panic!` will print a failure message, unwind, clean up the stack, and `quit`. 

There are two ways to cause a panic in practice: 
- by taking an action that causes our code to panic (such as accessing an array past the end) 
- or by explicitly calling the `panic!` macro. 

### RUST_BACKTRACE environment variable

We can set the `RUST_BACKTRACE` environment variable to get a `backtrace` of exactly what happened to cause the error. 

 A `backtrace` is a list of all the functions that have been called to get to this point. 

## Recoverable Errors with Result

Most errors aren’t serious enough to require the program to stop entirely. 

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is defined as having two variants, `Ok` and `Err`.

- `T` represents the type of the value that will be returned in a `success` case within the `Ok` variant.
- `E` represents the type of the error that will be returned in a `failure` case within the `Err` variant. 

### unwrap and expect

For `unwrap()`:
- If the Result value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. 
- If the Result is the `Err` variant, `unwrap` will call the `panic!` macro for us.

The `expect()` method lets us also choose the `panic!` error message.

### Propagating Errors

When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as `propagating` the error.

### ? operator

The `?` operator placed after a `Result` value is defined to work in almost the same way as the match expressions we defined to handle the Result values:
- If the value of the Result is an `Ok`, the `value inside` the Ok will get returned from this expression.
- If the value is an `Err`, the Err will be returned from the whole function as if we had used the `return` keyword.

`?` operator can be used with `Option<T>` values as well.
- If the value is `Some`, the value `inside` the Some is the resulting value of the expression.
- If the value is `None`, the `None` will be returned early from the function at that point.

The `?` operator can `only` be used in functions whose return type is `compatible` with the value the `?` is used on. 

You can 
- use the `?` operator on a `Result` in a function that returns `Result`.
- use the `?` operator on an `Option` in a function that returns `Option`, 
- `not mix` and match.

## Function main

The `main` function can also return a `Result<(), E>`.

`Box<dyn Error>` type is a trait object, which we’ll talk about later. For now, you can read `Box<dyn Error>` to mean “any kind of error.”

## To panic! or Not to panic!

In situations such as `examples`, `prototype code`, and `tests`, it’s more appropriate to write code that `panics` instead of returning a `Result`. 

The `unwrap` and `expect` methods are very handy when `prototyping`, `before` you’re ready to decide how to handle errors.

- The `panic!` macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values.
- The `Result` enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. 
