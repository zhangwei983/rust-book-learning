An example shows how traits works in Rust.

A trait defines functionality a particular type has and can share with other types.

## Defining a Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

We declare a trait:
- using the `trait` keyword and then the trait’s name, which is Summary in this case. 
- as `pub` so that crates depending on this crate can make use of this trait.
- with `method signatures` to describe the behaviors of the types that implement this trait.

The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

## Implementing a Trait on a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

One `restriction` to note is that 
- we can implement a trait on a type `only if` at least one of the trait or the type is local to our crate.
- we can’t implement `external traits` on `external types`.

This restriction is part of a property called `coherence`, and more specifically the `orphan rule`, so named because the parent type is not present. 

This rule ensures that other people’s code can’t break your code and vice versa.

## Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. 

Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

## Traits as Parameters

We can use traits to define functions that accept many different types.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

What we do:
- We specify the `impl` keyword and the `trait name`. 
- This parameter accepts `any type` that `implements` the specified trait. 

### Trait Bound

`&impl Summary` is syntax sugar for `Trait Bound`, like below

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
