An example of guessing game which is a hands-on way to introduce many new Rust concepts: `let`, `enum`, `match`, `mut`, `functions`, `external crates`, and more. 

Things to notice:

1. Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the [prelude](https://doc.rust-lang.org/stable/std/prelude/index.html).

2. Add `use std::io` to bring the `io` input/output library into scope.

3. Use the `let` statement to create the variable.

4. In Rust, `variables` and `references` are immutable by default.

5. Add `mut` to make a variable mutable.

6. [String](https://doc.rust-lang.org/stable/std/string/struct.String.html) is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

7. An associated function is a function that’s implemented on a type, e.g `String::new()`.

8. `io::stdin()` returns an instance of `Stdin`.

9. The `&` indicates that `guess` argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

10. An [enumeration](https://doc.rust-lang.org/stable/book/ch06-00-enums.html), often called an `enum` which is a type that can be in one of multiple possible states, each possible state is called a variant.

11. [Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html) is an enum, with variants `Ok` and `Err`.
    - The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. 
    - The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.
    - An instance of `Result` has an [expect method](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect) that you can call.
    - If this instance of `Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to expect.

12. The right way to suppress the warning is to actually write error-handling code, in our case we just use `expect` to simply crash this program when a problem occurs.
    ```rust
    // With error handling.
    match io::stdin().read_line(&mut guess) {
        Ok(count) => {
            println!("{count} bytes read");
            println!("{guess}");
        }
        Err(err) => {
            println!("error: {err}");
        }
    }
    ```

13. Add `rand` library to the dependencies in Cargo.toml.
    ```rust
    [dependencies]
    rand = "0.8.5"
    ```
    
14. Check for https://crates.io/ for open source Rust projects.

15. Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

16. `rand::thread_rng()` function gives us the particular random number generator
     - is local to the current thread of execution.
     - is seeded by the operating system.

17. Range expression https://rustwiki.org/en/reference/expressions/range-expr.html

18. Use `RUST_BACKTRACE=1` & `RUST_BACKTRACE=full` environment variables to display a backtrace.

19. The [Ordering](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html) type is another enum and has the variants `Less`, `Greater`, and `Equal`.

20. A [match expression](https://doc.rust-lang.org/stable/book/ch06-02-match.html) is made up of arms. An `arm` consists of a `pattern` to match against, and the code that should be run if the value given to match fits that arm’s pattern. 

21. `Shadowing` lets us reuse a variable name rather than forcing us to create two unique variables.

