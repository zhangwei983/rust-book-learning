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
