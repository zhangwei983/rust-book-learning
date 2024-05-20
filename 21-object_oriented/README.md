An example to show the object oriented programming features of Rust.

## Characteristics of Object-Oriented Languages

Rust is influenced by many programming paradigms, including OOP. OOP languages share certain common characteristics, namely `objects`, `encapsulation`, and `inheritance`. 

| Feature        | Supported   | Description   |
| -------        | ----------- | -----------   |
| Objects        | &check;     | `Structs` and `enums` have data, and `impl` blocks provide methods on structs and enums. |
| Encapsulation  | &check;     | Using `pub` or not enables encapsulation of implementation details.
| Inheritance    | &cross;     | Rust doesn't support inheritance. But for `reusing code`, Rust provides `default` trait method implementations to achieve it in a limited way. For `polymorphism`, Rust uses `generics` and `trait bounds`.|

## Traits object

Below is an example for trait objects.

```rust

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Here `Box<dyn Draw>` is a trait object. It stands for any type inside a `Box` that implements the `Draw` trait.

Trait objects
- are more like objects in other languages in the sense that they combine data and behavior. 
- differ from traditional objects in that we can’t add data to a trait object. 

In the above sample, the `components` vector contains a list of objects that implement `Draw` trait. With this, we can achieve similar behavior of `polymorphism` in other languages.

When we use trait objects, 

- Rust must use `dynamic dispatch`. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call.
- At runtime, Rust uses the `pointers` inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn’t occur with static dispatch. 
- Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations. 

However, we get extra `flexibility`!

## Traits object vs. Trait Bounds

Below is an example of trait bounds.

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

In this case, the generic type parameter can only be substituted with `one` concrete type at a time. This restricts the `Screen` instance can only hold one type of component, e.g. all of type Button or all of type TextField. 

This works differently from trait objects, which allow for `multiple` concrete types to fill in for the trait object at runtime.
