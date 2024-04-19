A example to show the basic usage of struct.

## Struct

An example of struct

```rust

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Mutable 

Limitation:

- Then `entire` instance must be mutable or not.
- Rust doesn’t allow us to mark only certain fields as mutable. 

### Struct Update Syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

Be careful about `Move`! 

In above sample, `user1.username` will be moved to `user2.username`, you cannot use `user1.username` anymore.

## Tuple Structs

Tuple structs doesn't have field names, looks similar to `tuple`.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

Even struct `Color` and `Point` contain exact the same fields, they're not same types.

## Unit-Like Structs

Unit-Like structs:

- don't have any fields.
- can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

```rust
struct AlwaysEqual;
```

We will describe the trait usage later.
