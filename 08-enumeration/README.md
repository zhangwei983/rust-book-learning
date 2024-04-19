An example to show the basic usage of Enum.

Enums give you a way of saying a value is one of a possible set of values.

## Definig an Enum

The below is an example.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

- Data can be put directly into each enum variant. 
- Each variant can have different types and amounts of associated data.
- enum can have function implemeted like struct, in `impl` block.

```rust
#[derive(Debug)]
enum Message {
    Quit,                       // No data associated.
    Move {x: i32, y: i32},      // Has named fields, like a struct.
    Write(String),              // Include a single String.
    ChangeColor(i32, i32, i32), // Includes 3 i32 values.
}

impl Message {
    fn call(&self) {
        ...
    }
}
```

Please check the [IpAddr](https://doc.rust-lang.org/stable/std/net/enum.IpAddr.html) defined in std library.

## Option Enum

`Option` 

- is an enum defined by the standard library.
- encodes the very common scenario in which a value could be `something` or it could be `nothing`.

`Option<T>` is [defined by the standard library](https://doc.rust-lang.org/stable/std/option/enum.Option.html) as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `Option<T>` enum is so useful that:
- it’s even included in the `prelude`.
- its variants are also included in the prelude: you can use `Some` and `None` directly without the `Option:: prefix`.
- `<T>` is a generic type parameter, it means that the `Some` variant of the `Option` enum can hold one piece of data of `any type`.

### None vs Null Value

A null is a value that is currently invalid or absent for some reason.

Rust:

- does `not` have nulls
- but it does have `None` that can encode the concept of a value being `absent`.
- `None` value, in some sense it means the same thing as null. 

Why is having Option<T> any better than having null?

- In short, because `Option<T>` and `T` (where T can be any type) are `different` types.
- The compiler won’t let us use an `Option<T>` value as if it were definitely a `valid` value. 

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```

Analyses:

- This code will not compile, as `Option<i8>` and `i8` are different types.
- When we have an `Option<i8>`, we have to worry about possibly not having a value.
- And the compiler will make sure we handle that case before using the value.
- You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it.

This was a `deliberate` design decision for Rust to limit `null’s pervasiveness` and increase the safety of Rust code.

### Get Value from Some Varaint

How do you get the T value out of a Some variant when you have a value of type Option<T>?

You can check useful methods on `Option<T>` in [its document](https://doc.rust-lang.org/stable/std/option/enum.Option.html). Becoming familiar with the methods on Option<T> will be **extremely** useful in your journey with Rust.


## Expression `match`

Rust has an extremely powerful control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. 

Patterns can be made up of literal values, variable names, wildcards, and many other things. Check [this document](https://doc.rust-lang.org/stable/book/ch18-00-patterns.html) for more details.

`match` 
- can return any type.
- the arms’ patterns must cover `all` possibilities.

### Match Option<T>

Another useful feature of `match arms` is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

This is an example of matching `Option<i32>`.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

`Some(i) => Some(i + 1)` extracts the value from `Some(T)`, and bind it to `i`.

You’ll see this pattern a lot in Rust code: 

- `match` against an enum
- bind a `variable` to the data inside
- and then execute code based on it.

### Catch-all pattern

`other` pattern is used to cover every other possible value.

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
```

The above sample shows how `other` covers all the other possble values beside 3 and 7.

`_` is a special pattern that matches any value and does not bind to that value.

```rust
let dice_roll = 9;

match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
```

### With if let

You can:
- think `if let` is a syntax sugar for a `match`.
- include an `else` with an `if let`.

```rust
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

