An example to show the usage of some basic collections in Rust.


The data these collections point to is stored on the `heap`, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. 

## Vector

A vector allows you to store a variable number of values next to each other.

Vectors are implemented using generics. Rust will try to infer the type of value you want to store.

```rust
    let v: Vec<i32> = Vec::new();
```

Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it. 

```rust
let v = vec![1, 2, 3];
```

Rust infers v as `Vec[i32]`.

### Reading Elements

You can use [] or get() method.

```rust
let third = &v[2];

let third = v.get(2);
```

The difference is if index is out of range:
- `[]` will panic.
- but `Vec<T>.get()` will return an Option that you can handle the error.


The below code won't compile as the mutable borrow happens while an immutable borrow still exists.

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```

Why should a reference to the first element care about changes at the end of the vector?

This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. 

### Iterating over the Values

```rust
for i in &v { // Iterate values.

for i in &mut v { // Iterate mutable values.
```

### Using an Enum to Store Multiple Types

The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!

Rust needs to know what types will be in the vector at `compile time` so it knows exactly how much memory on the `heap` will be needed to store each element. 

## String

## HashMap
