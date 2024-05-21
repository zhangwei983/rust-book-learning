An example to show how pattern matching works in Rust.

## Pattern Matching

### Basic Pattern Matching

A typical match pattern using `match` expression with multiple arms.

```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

### Conditional `if let`

Use `if let` to match for one case.

```rust
if let Some(i) = x {
    println!("x: {}", i);
}
```

### Conditional Loop `while let`

Use `while let` to loop as long as the pattern continues to match.

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### for Loops

The value that directly follows the keyword for is a pattern.

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

`(index, value)` is the pattern in the above example.

### let Statements

```rust
let PATTERN = EXPRESSION;
```

Every time you've used a `let` statement like this you've been using patterns, although you might not have realized it.

```rust
let (x, y, z) = (1, 2, 3);
```

### Function Parameters

The function parameter can also be a pattern.

```rust
fn foo(x: i32) {
    // code goes here
}
```

The `x` part is a pattern! 

## Refutability

Patterns come in two forms: `refutable` and `irrefutable`. Please check the example [here](./src/refutability.rs).

### Irrefutable

Patterns that will match for any possible value passed are `irrefutable`. 

The `let` statements are irrefutable. For example, `let x =5`, `x` matches anything and therefore cannot fail to match.

`Function parameters`, `let` statements, and `for` loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. 

### Refutable

Patterns that can fail to match for some possible value are refutable. 

For example, `if let Some(x) = a_value`, `Some(x)` is refutable as the value in the `a_value` variable might be `None`.

The `if let` and `while let` expressions accept `refutable` and `irrefutable` patterns, but the compiler warns against irrefutable patterns.


## Pattern Syntax

Please check [this example](./src/pattern_syntax.rs) for pattern syntax.

### Basic Matching

Match multiple patterns.

```rust
let x = 1;
match x {
    1 | 2 => println!("One or two"),
    _ => println!("Other"),
}
```

Match range.

```rust
let x = 5;
match x {
    1..=5 => println!("One to five"),
    _ => println!("Other"),  
}
```

### Destructure Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);
```

Creates the variables `a` and `b` that match the values of the x and y fields of the point struct.

### Destructure Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.");
    }
    Message::Move { x, y } => {
        println!("Move in the x direction {x} and in the y direction {y}");
    }
    Message::Write(text) => {
        println!("Text message: {text}");
    }
    Message::ChangeColor(r, g, b) => {
        println!("Change the color to red {r}, green {g}, and blue {b}",)
    }
}
```

We can destructure enums to retrive the data stored on it.

### Ignore Values

There are a few ways to ignore entire values or parts of values in a pattern: 
- using the `_` pattern
- using the `_` pattern within another pattern
- using a name that starts with an `underscore`, 
- using `..` to ignore a certain range parts of a value.

### Match Guards

A match guard is an additional `if` condition, specified after the pattern in a match arm, that must also match for that arm to be chosen. 

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

The above match has its first arm with the pattern `Some(x)` and also a `match guard` of `if x % 2 == 0`.


### Match Bindings

Use the `at` operator `@` to create a variable that holds a value at the same time as we’re testing that value for a pattern match.

```rust
enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
```
