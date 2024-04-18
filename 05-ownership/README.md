
In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. 

## Ownership Rules

Ownership Rules:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### String type
`String` type is used to illustrate the rules of ownership.

In order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap for `String`.

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.
  - In languages with a garbage collector (GC), the `GC` keeps track of and cleans up memory that isn’t being used anymore
  - In most languages without a GC, it’s `our responsibility` to identify when memory is no longer being used and to call code to explicitly free it.
  - In Rust, the memory is automatically returned once the variable that owns it goes `out of scope`.


In Rust

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

### Move

The below code will get compiling errors.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

- `let s2 = s1`  will `not` make a copy of the value in `s1` and bind it to `s2`.
- To ensure memory safety, after `let s2 = s1;`, Rust will consider `s1` as no longer valid
    - To avoid double memory free;
    - Otherwise, when `s2` and `s1` go out of scope， they will both free the memory on the heap where the actual string content is stored.

### Move vs Shallow Copy

Compared to `Shallow Copy` in other languages like C++, `Move` in Rust also `invalidate` the first variable who is being shallow-copied.

And Rust will never `automatically` do `Deep Copy` on your heap data.

### Clone

If you do want to do `Deep Copy` on the heap data of the String, please use a common method called `clone` instead. 

### Copy trait

If a type implements the `Copy` trait, variables that use it 

- do **not** move, 
- but rather are trivially copied, making them still valid after assignment to another variable.

As a general rule,

- any group of `simple scalar` values
- nothing that requires allocation
- some form of resource can implement Copy

can implement `Copy`.

Here are some of the types that implement Copy:

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

### Ownership and Functions

- Passing as input argument
- Returning as result

The mechanics of passing/returning a value to a function are similar to those when assigning a value to a variable.

## References and Borrowing

A reference:

- is like a pointer in that it’s an address we can follow to access the data stored at that address.
- is guaranteed to point to a valid value of a particular type for the life of that reference.
- allows you to refer to some value without taking ownership of it.
- the scope starts from where it is introduced and continues through the last time that it is used.

Borrowing is the action of creating a reference.

Immutable references

- allow you to have multiple immutable references.

Mutable references

- allow you to modify the borrowed variable.
- You can only have one mutable reference.
- the scopes of mutable and immutable references cannot overlap.

## Slice Type

A slice:

- allows you reference a contiguous sequence of elements in a collection.
- is a kind of reference.
- doesn't have ownership.

### String slice

`&str` is an immutable reference.

``` rust
let s = "Hello world"
```

Here `s` is a String slice (`&str`),  pointing to that specific point of the binary.

`&String` can be implicitly converted to `&str` by [Deref Coercions](https://doc.rust-lang.org/stable/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods).
