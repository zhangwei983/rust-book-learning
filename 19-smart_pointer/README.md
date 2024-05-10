An example to show the usage of Smart Pointers in Rust.

## Box<T>

The most `straightforward` smart pointer is a `box`, whose type is written `Box<T>`. Boxes allow you to store data on the `heap` rather than the stack.

Boxes provide `only` the indirection and heap allocation; they don’t have any other special capabilities.

It can be used  most often in these situations:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

```rust
let b = Box::new(5);
```

The Box<T> type is a smart pointer because it implements the `Deref trait`, which allows Box<T> values to be treated like references.

When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop trait` implementation.

### Enabling Recursive Types with Boxes

A value of `recursive` type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.

Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

...

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

```

## Deref Trait

Implementing the `Deref trait` allows you to customize the behavior of the dereference operator `*`.

Without the `Deref` trait, the compiler can only dereference `&` references. 

```rust
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
With `Deref trait` implemented on `MyBox<T>`, you can use `*` as below:

```rust
let x = MyBox::new(x);
assert_eq!(5, *x);
```

Here for `*x`, Rust actually runs the code `*(x.deref())` behind. Rust substitutes the `*` operator with a call to the `deref` method and then a `plain dereference`.

The reason the `deref` method returns a `reference` to a value is to do with the `ownership` system. If the deref method returned the `value` directly instead of a reference to the value, the value would be moved out of `self`. 

> [!NOTE]  
> Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator `just once`.

### Implicit Deref Coercions

`Deref coercion` converts a reference to a type that implements the `Deref` trait into a reference to another type.

For example, `deref coercion` can convert `&String` to `&str` because String [implements](https://doc.rust-lang.org/std/string/struct.String.html#deref) the `Deref trait` such that it returns `&str`. 

Similar to how you use the `Deref` trait to override the `*` operator on `immutable` references, you can use the `DerefMut` trait to override the `*` operator on `mutable` references.

Rust does deref coercion when it finds types and trait implementations in three cases:

- From &T to &U when T: Deref<Target=U>
- From &mut T to &mut U when T: DerefMut<Target=U>
- From &mut T to &U when T: Deref<Target=U>  
  Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references.

## Drop Trait

The `Drop` trait lets you customize what happens when a value is about to go `out of scope`. For example, when a `Box<T>` is dropped it will deallocate the space on the heap that the box points to.

The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to self. 

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

### Dropping a Value Early with std::mem::drop

Occasionally, you might want to clean up a value early. One example is when using smart pointers that manage `locks`: you might want to force the drop method that releases the lock so that other code in the same scope can acquire the lock. 

Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the `std::mem::drop` function provided by the standard library.

## Rc<T>, the Reference Counted Smart Pointer

There are cases when a single value might have `multiple` owners. You can enable multiple ownership `explicitly` by using the Rust type `Rc<T>`, which is an abbreviation for `reference counting`.

The `Rc<T>` type keeps track of the number of `references` to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

> [!NOTE]  
> Note that `Rc<T>` is only for use in `single-threaded` scenarios.

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

The implementation of [Rc::clone](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.clone) doesn’t make a `deep copy` of all the data like most types’ implementations of clone do. `Rc::clone` only creates another pointer to the same allocation, and increase the `strong reference count`.

`Rc::strong_count` function can be called to return the strong reference count. This function is named `strong_count` rather than count because the `Rc<T>` type also has a `weak_count`, which we will talk about later.

Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

## RefCell<T> and the Interior Mutability Pattern

`Interior mutability` is a `design pattern` in Rust that allows you to `mutate` data even when there are `immutable references` to that data.

To `mutate` data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. `Unsafe` code indicates to the compiler that we’re checking the rules `manually` instead of relying on the compiler to check them for us.

Recall the borrowing rules:
- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References must always be valid.

With 
- `references and Box<T>`, the borrowing rules’ invariants are enforced at `compile` time. If you break these rules, you’ll get a compiler error. 
- `RefCell<T>`, these invariants are enforced at `runtime`. If you break these rules, your program will panic and exit.

> [!NOTE]  
> Note that `RefCell<T>` is only for use in `single-threaded` scenarios and will give you a compile-time error if you try using it in a multithreaded context. 

Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at `compile` time; `Rc<T>` allows only immutable borrows checked at `compile` time; RefCell<T> allows immutable or mutable borrows checked at `runtime`.
- Because `RefCell<T>` allows mutable borrows checked at `runtime`, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

### Interior Mutability: A Mutable Borrow to an Immutable Value

The `RefCell<T>`: 
- provides the `borrow` method to return a smart pointer type `Ref<T>`, to have `immutable` references.
- provides the `borrow_mut` method to return a smart pointer type `RefMut<T>`, to have `mutable` references.
- keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active.  

Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time. If we violate these rules, RefCell<T> will `panic` at runtime rather than getting a compiler error.

Both `Ref<T>` amd `RefMut<T>` types implement `Deref`, so we can treat them like regular references.

## Combining Rc<T> and RefCell<T>

A common way to use `RefCell<T>` is in combination with `Rc<T>`. You can get a value that can have multiple owners and that you can mutate by having an `Rc<T>` that holds a `RefCell<T>`.

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

...

let value = Rc::new(RefCell::new(5));
let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

*value.borrow_mut() = 10;

...
```

In the above example, we call `borrow_mut` on `value` to change its value. The `borrow_mut` method returns a `RefMut<T>` smart pointer, and we use the `dereference operator` on it and change the inner value.

## Reference Cycle

Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. 

Check the example in [ref_cycles.rs](./src/ref_cycle.rs).

### Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

TODO.