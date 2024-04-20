An example to show the generic data types in Rust.

`Generics` is abstract that stand-ins for concrete types or other properties.

## In Function Definitions

```rust
fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    ...
}
```

We read this definition as: 
- this function `find_largest` is `generic` over some `type T`. 
- this function has one parameter named list, which is a `slice` of values of `type T`. 
- this function will return a `reference` to a value of the same `type T`.
- T is restricted to types that implement `std::cmp::PartialOrd` trait.  
  The standard library has the `std::cmp::PartialOrd` trait that you can implement on types to enable `comparisons`.

## In Struct Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

The syntax for using `generics` in `struct definitions` is similar to that used in `function definitions`. 
- We declare the name of the type parameter inside `angle brackets` just after the name of the struct. 
- Then we use the generic type in the struct definition where we would otherwise specify concrete data types.

## In Enum Definitions

We can define enums to hold generic data types in their variants.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is generic over `type T` and has two variants: 
- `Some`, which holds one value of type T
- `None` variant that doesn’t hold any value. 

Enums can use `multiple` generic types as well. 

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, T and E, and has two variants: 
- `Ok`, which holds a value of type T.
- `Err`, which holds a value of type E.

## In Method Definitions

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

We have to declare T just after `impl` so we can use T to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after `impl`, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.

### Constraints on generic types

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

So only `Point<f32>` has `distance_from_origin` method implemented.

## Performance of Code Using Generics

Using generic types won't make your program run any slower than it would with concrete types.

1. Rust accomplishes this by performing `monomorphization` of the code using generics at `compile time`.
2. `Monomorphization` is the process of turning generic code into specific code by `filling in the concrete types` that are used when compiled.
3. The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. 

The original version.
```rust
let integer = Some(5);
let float = Some(5.0);
```

The monomorphized version

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
