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

### Procedural Macros

The `procedural macro` acts more like a function (and is a type of procedure).

Procedural macros:
- accept some code as an input,
- operate on that code,
- and produce some code as an output.

When creating `procedural macros`, the definitions must reside in their own crate with a special crate type.

### Write a Custom derive Macro

We want to achieve something like below, with `HelloMacro` defined as a `derive Macro`. 

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

And `derive` only works for `structs` and `enums`.

#### For `macros_derive` Crate

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

#### For `macros` Crate

In `macros` crate, we need to
- add `macros_derive` as a dependency, please refer to [here](./macros/Cargo.toml#L9).
- Define a trait with the same name as the procedure macro, please check [here](./macros/src/lib.rs#L35).

These two crates are tightly related. If you change the [trait definition](./macros/src/lib.rs#L35) in `macros`, you have to change the [implementation](./macros_derive/src/lib.rs#L6) of the procedural macro in `macros_derive` as well. 

### Attribute-like Macros

Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes.

Attributes can be applied to items other than structs and enums, such as functions.

Below is an example of an attribute.

```rust
#[route(GET, "/")]
fn index() {
```

This #[route] attribute would be defined by the framework as a procedural macro as below.

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

There are two `TokenStream` parameters:
- The first is for the contents of the `attribute`: the `GET, "/"` part. 
- The second is the `body` of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.

### Function-like Macros

Function-like macros define macros that look like function calls.

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

The `sql!` macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

The `sql!` macro can parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a `macro_rules!`.

### cargo-expand

The [cargo-expand](https://lib.rs/crates/cargo-expand) is a library that can show the result of macro expansion.

Please run the below command to install:

```bash
cargo install cargo-expand
```

For below code, 

```rust
#[derive(Debug)]
struct S;

fn main() {
    println!("{:?}", S);
}
```

If you run

```bash
cargo expand
```

you will get

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
struct S;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            S => {
                let mut debug_trait_builder = f.debug_tuple("S");
                debug_trait_builder.finish()
            }
        }
    }
}
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&S,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
}
```

Run `cargo expand --help` to check more details.
