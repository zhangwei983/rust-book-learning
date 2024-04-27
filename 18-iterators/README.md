An example to show the iterator usage in Rust.

The iterator pattern allows you to perform some task on a `sequence` of items `in turn`. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. 

In Rust, iterators are `lazy`, meaning they have no effect until you call methods that `consume` the iterator to use it up.

The below is an example of an iterator.

```rust
let v1 = vec![1, 2, 3];

let iter_v1 = v1.iter();
```

## The Iterator Trait and the next Method

All iterators implement a `trait` named `Iterator` that is defined in the standard library. The definition of the `Iterator` trait looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

The `Iterator` trait only requires implementors to define one method: the `next` method, which returns: 
- one item of the iterator at a time wrapped in `Some`,
- `None` when iteration is over.

The `type Item` and `Self::Item`, which are defining an `associated type` with this trait. It means implementing the `Iterator` trait requires that you also define an `Item` type, and this `Item` type is used in the `return` type of the `next` method.

## Methods that Consume the Iterator

The `Iterator` trait has a number of different methods with `default implementations` provided by the standard library. 

Some of these methods call the `next` method in their definition, which is why you’re `required` to implement the `next` method when implementing the `Iterator` trait.

Methods that call `next` are called `consuming adaptors`, because calling them uses up the iterator.

## Methods that Produce Other Iterators

`Iterator adaptors`
- are methods defined on the `Iterator` trait that `don’t` consume the iterator. 
- produce `different iterators` by changing some aspect of the original iterator instead.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```

The `map` method is an `iterator adaptor`, which 
- takes a ·closure· to call on each item as the items are iterated through.
- returns a ·new iterator· that produces the `modified items`.

The above iterator doesn’t do anything: 
- the closure specified `never` gets called. 
- iterator adaptors are `lazy`, and we need to consume the iterator here.

```rust
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

So in the above code:
- the `map` method creates a `new` iterator.
- the `collect` method consumes the `new` iterator.

Here `map` takes a `closure`, we can specify `any operation` we want to perform on `each item`. 

This is a great example of how closures let you `customize` some behavior while reusing the `iteration behavior` that the `Iterator` trait provides.

## Using Closures that Capture Their Environment

Many `iterator adapters` take `closures` as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that `capture` their environment.

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

In the body of `shoes_in_size`, we call 
- `into_iter` to create an `iterator` that `takes ownership` of the vector. 
- call `filter` to adapt that iterator into a `new` iterator that only contains elements for which the closure returns true.

The closure captures the `shoe_size` parameter from the `environment` and compares the value with each shoe’s size.
