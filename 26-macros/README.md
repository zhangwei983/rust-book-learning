An example to show the usage of macros in Rust.

## Macros

The term `macro` refers to a family of features in Rust: 
- Declarative macros with macro_rules! 
- Three kinds of procedural macros:
    - Custom #[derive] macros  
      Specifies code added with the derive attribute used on structs and enums.
    - Attribute-like macros
      Defines custom attributes usable on any item.
    - Function-like macros
      Looks like function calls but operate on the tokens specified as their argument.

### The Difference Between Macros and Functions

Macros are a way of writing code that writes other code, which is known as `metaprogramming`.

|                 | Macros                                   | Functions                                       |
|-----------------| -----------------------------------------|-------------------------------------------------|
| Signature       | Take a variable number of paramters      | Must declare the number and type of parameters  |
| Process time    | Expanded at compiling time               | Gets called at runtime                          |
| Easy to use?    | Complex                                  | Simple                                          |
| Scope           | Define or bring into scope before using  | Define and call anywhere                        |

Macros are expanded at compiling time, so a macro can implement a trait on a given type.

### Declarative Macros with macro_rules!

Declarative macros allow you to write something similar to a Rust `match` expression.

Macros compare a value to patterns that are associated with particular code:
- the value is the literal Rust source code passed to the macro;
- the patterns are compared with the structure of that source code;
- when matched, the code associated with each pattern replaces the code passed to the macro.

```rust
#[macro_export]
macro_rules! my_vec {
    ( $($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The above is an example of defining a declarative mccro with `macro_rules!`.
- The `#[macro_export]` annotation indicates the macro is available when the parent crate is brought into scope.
- The macro is defined without the `exclamation` mark.
- The structure in the macro body is similar to the structure of a `match` expression.

### Write a Custom derive Macro

We want to achieve something like below, with `HelloMacro` defined as a derive Macro. 

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

At the time of this writing, procedural macros need to be in their own crate. This is why we created a separate [lib crate](./macros_derive/) in our example.

The convention for structuring crates and macro crates is as follows: 
- for a crate named `macros`, 
- a custom derive procedural macro crate is called `macros_derive`. 

#### For macros_derive
And for `macros_derive`, you need to add following content to the `Cargo.toml` file:

```
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

For `proc-macro`, it indicates the library is a procedure macro crate. Please check the [Cargo document](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-proc-macro-field) for details.

For the macro definition itself, please refer to the [lib.rs](./macros_derive/src/lib.rs) file.

#### For macros

In `macros` crate, we need to
- add `macros_derive` as a dependency, please refer to [here](./macros/Cargo.toml#L9).
- Define a trait with the same name as the procedure macro, please check [here](./macros/src/lib.rs#L35).

These two crates are tightly related. If you change the [trait definition](./macros/src/lib.rs#L35) in `macros`, you have to change the [implementation](./macros_derive/src/lib.rs#L6) of the procedural macro in `macros_derive` as well. 
