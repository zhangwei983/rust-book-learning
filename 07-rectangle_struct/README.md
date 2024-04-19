An example to show some advanced features of struct in Rust.

## Struct Display

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

In Rust, structs don’t have a `default` implementation of `std::fmt::Display` to use with println! and the {} placeholder.

With structs, the way `println!` should format the output is not clear because there are more display possibilities: 
- Do you want commas or not? 
- Do you want to print the curly brackets? 
- Should all the fields be shown?


### Derive Debug Trait

Enable `Debug` trait on `Rectangle` like below

```rust
#[derive(Debug)]
struct Rectangle {
    ...
}
```

### Output

- Use the specifier `:?` or `{:#?}` to  tell `println!` to use `Debug` output format.  
  `println!` prints to the standard output console stream (`stdout`).

- Another way to print out a value using the `Debug` format is to use the [dbg! macro](https://doc.rust-lang.org/stable/std/macro.dbg.html).  
  `dbg!` macro prints to the standard error console stream (`stderr`)

## Struct Methods

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- `impl` starts the `implementation` block of Rectangle.
- `&self` is a short for `self: &Self`.
- `Self` is an alias for the type of `impl` block for, here is `Rectangle`.
- Multiple `impl` Blocks is supported.

So 

```rust
    fn area(self: &Rectangle) -> u32 {
        ...
    }
```
is right too.

## Associated Functions

All functions defined within an `impl` block are called `associated functions` because they’re associated with the `type` named after the `impl`.

Associated functions that `aren’t` methods
- `don’t` have `self` as their first parameter.
- are often used for `constructors` that will return a new instance of the struct. 
- e.g., `String::from()`.
