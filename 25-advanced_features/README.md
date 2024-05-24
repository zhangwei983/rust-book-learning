An example to show some advanced features in Rust.

## Unsafe Rust

The unsafe code should be included by a new block started with the `unsafe` keyword. It can be used to:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

Please check the example at [here](./src/unsafe_rust.rs).

### Dereferencing a Raw Pointer

`Raw` pointers:

- ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location.
- aren’t guaranteed to point to valid memory.
- can be null.
- have no automatic cleanup.

```rust
let mut num = 5;

let r2 = &mut num as *mut i32;

unsafe {
    println!("r2 is: {}", *r2);
}
```

In the above example, use the `dereference operator *` on a raw pointer that requires an `unsafe` block.

### Calling an Unsafe Function or Method

An `FFI` (Foreign Function Interface) is a way for a programming language to define functions which can be called from a different (foreign) programming language.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Below is an example to make the `call_from_c` function accessible from C code, after it’s compiled to a shared library and linked from C.

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

`[no_mangle]` annotation to tell the Rust compiler not to `mangle` the name of this function.

## Advanced Traits

### Associated Types

`Associated types` connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The `type` Item is a placeholder in the above example.

#### Comparing to Generic Types

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is:

- With `generics`, you can have `multiple` implementation of `Iterator` for `Counter`.
  For example, `impl Iterator<u32> for Counter` or `impl Iterator<String> for Counter`.
- With `associated types`, you can choose only `one` type of Item as there can only be one `impl Iterator` for `Counter`.

### Default Generic Types

Below is an example of the default generic type within the `Add` trait.

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

By default, `add` will add two instances with the same type `Self`. But you can specify the `Rhs` to another type to add two instances with different types.

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

In the above example, `Rhs` is specified to `Meters`, then in the `add` function, we can add two instances with `Meters` and `Millmeters` types.

### Fully Qualified Syntax

```rust
trait Pilot {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Flying as a pilot.");
    }
}

impl Human {
    fn fly(&self) {
        println!("Flying as a human.");
    }
}
```

You can call different `fly` methods as below.

```rust
let human = Human;
Pilot::fly(&human);
human.fly();
```

#### For Associated Functions

Associated functions don’t have a `self` parameter. 

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
```

You can call different associated functions as below

```rust
println!("A baby dog is called a {}", Dog::baby_name());
println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
```

### Using Supertraits

You can write a trait definition that depends on another trait.

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        //...
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //...
    }
}
```

In the above example, we define an `OutlinePrint` trait that depends on `fmt::Dispaly` trait. Then the compiler will ask you to implement `fmt::Display` for `Point` if you forget.
