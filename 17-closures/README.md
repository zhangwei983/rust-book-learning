An example to show how closures work in Rust.

## What is a Closure

In Rust, closures 
- are `anonymous` functions you can save in a variable or pass as arguments to other functions.
- closures can `capture values` from the scope in which they’re defined. 

## Declare Closure

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value. 

- The first line shows a `function` definition. 
- And the second line shows a `fully` annotated `closure` definition. 
- In the third line, we remove the type annotations from the closure definition. 
- In the fourth line, we remove the brackets, which are optional because the closure body has only one expression.

The `add_one_v3` and `add_one_v4` lines require the closures to be `evaluated` to be able to compile because the types will be `inferred` from their usage.

```rust
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
```

The code won't compile, as the compiler will infer `x` as type `String`, so the `example_closure(5)` will cause compiling errors.

## Capturing References or Moving Ownership

Closures can `capture` values from their environment in three ways, which directly map to the three ways a function can take a parameter: 
- borrowing `immutably`.
- borrowing `mutably`.
- and taking `ownership`.

The closure will decide which of these to use based on what the body of the function does with the captured values.

```rust
let mut list = vec![1, 2, 3];

let mut mutable_borrow = || list.push(7);
```

When `mutable_borrow` is `defined`, it captures a mutable reference to list.

### The move Keyword

You can use the `move` keyword before the parameter list to `force` the closure to take ownership of the values, even though the body of the closure doesn’t strictly need ownership. 

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

We use the `move` keyword to let the new thread take the ownership of the list.


## The Fn Traits

A closure body can do any of the following: 
- move a captured value out of the closure, 
- mutate the captured value, 
- neither move nor mutate the value, 
- or capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. 

Closures will `automatically` implement one, two, or all three of these `Fn` traits, depending on how the closure’s body handles the captured values:

- `FnOnce` applies to closures that can be called `once`.   
  All closures implement `at least` this trait, because all closures can be called. 
- `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values.   
  These closures can be called `more than once`.
- `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.  
  These closures can be called `more than once` without mutating their environment.

Functions can implement all three of the `Fn` traits too. 

Below is the definition of the [unwrap_or_else](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else) method on `Option<T>`.

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

 The unwrap_or_else function has the `additional` generic type parameter `F`. The `F` type is the type of the parameter named `f`, which is the `closure` we provide when calling `unwrap_or_else`.

 The `trait bound` specified on the generic type `F` is `FnOnce() -> T`, which means `F` must be able to be called `once`, take no arguments, and return a T.

Using `FnOnce` in the `trait bound` expresses the constraint that `unwrap_or_else` is only going to call `f` at `most one time`. 

```rust
pub fn sort_by<F>(&mut self, mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    stable_sort(self, |a, b| compare(a, b) == Less);
}
```

The reason [sort_by_key](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.sort_by_key) is defined to take an `FnMut` closure is that it calls the closure `multiple times`: once for each item in the slice. 

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

The above code won't compile as sort_by_key require a closure implement the `FnMut` trait, but the closure here only implemented the `FnOnce` trait.

The closure:
- captures value then moves `value` out of the closure by `transferring` ownership of value to the `sort_operations` vector. 
- only implements `FnOnce`. 
- can be called `only once`  
  trying to call it a second time wouldn’t work because `value` would no longer be in the environment to be pushed into `sort_operations` again.

Change the closure as below, then it can implement the `FnMut` trait.
```rust
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
```

### What does `move captured values out of the closure` exactly mean?

My understanding:
- The closure captures String `value` into its environment.
- When sorting the list, the `r.width` will be called multiple times to compare. This means 
   ```rust
   sort_operations.push(value);
   ```
   will be called multiple times.  
   But when it's been called on the first time, it will be moved out of the closure by transferring ownership of value to the sort_operations vector.
- You can think of this as the closure is executed as an `anonymous` functions with all the variables it needs, on a `separate stack`. If some variables are moved out from the stack by by transferring ownership, then they cannot be used any more in the current function.
