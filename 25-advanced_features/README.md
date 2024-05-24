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
