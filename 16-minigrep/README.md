An example of a command line tool that interacts with file and command line input/output.

Basically this example contains a list of knowledges we've learnt from the previous examples, including
- struct
- String
- Vec
- Result
- module
- tests
- unwrap_or_else
- expect
- if let
- ...

along with some new features introduced, including

- std::env::args
- std::env::var
- std::fs::read_to_string
- Box<dyn Error>
- closure
- eprintln!

So here we will describe the new features

## std::env::args()

Normally we use [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html) function in `std` to access the command line arguments. It returns an `iterator` which can be used to iterate the arguments.

```rust
use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }
}
```

> [!NOTE]
> Note that `std::env::args` will `panic` if any argument contains `invalid Unicode`. If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead. That function returns an iterator that produces `OsString` values instead of String values. 

## std::env::var

Below is an example of how to use [std::env::var]((https://doc.rust-lang.org/std/env/fn.var.html#)).

```rust
let ignore_case = env::var("IGNORE_CASE").is_ok();
```

The `std::env::var` function:
- returns an `Ok` variant that contains the value of the environment variable if it's set.
- return an `Err` variant if the environment variable is not set.

The [Result::is_ok](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok) returns true if the result is `Ok`.

## fs::read_to_string

The function [std::fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) is used to read the content of a file into a `String`.

```rust
let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
```

## eprintln!

The `eprintln!` macro is provided by the standard library to print to the standard `error` stream.

```rust
eprintln!("Print to standard error stream.");
```

Meanwhile `println!` macro prints to the standard `output` stream.

## Improve Modularity and Error Handling

### Separation of Concerns for Binary Projects

Split your program into a `main.rs` and a `lib.rs`. It's about separating concerns: 
- main.rs handles running the program, 
- and lib.rs handles all the logic of the task at hand. 

A call to panic! is more appropriate for a programming problem than a usage problem

unwrap_or_else


Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be.

if let

env::var

eprintln!