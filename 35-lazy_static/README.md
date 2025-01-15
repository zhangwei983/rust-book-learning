An example to show the basic usage of the [lazy_static crate](https://crates.io/crates/lazy_static).

## What is `lazy_static`
The `lazy_static` macro allows you to define static variables that are initialized lazily. 

The variables are 
- initialized when they are `first accessed`,
- and the initialization code is only run `once`,
- `immutable`, but if you need to modify them, you can use a `Mutex` to make them mutable.

## Use `cargo expand` to see the generated code

The `cargo expand` command can be used to show the generated code.

```shell
cargo expand
```

The following code:

```rust
lazy_static! {
    static ref NUMBER: u32 = 21;
}
```

will be expanded to:

```rust
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct NUMBER {
    __private_field: (),
}
#[doc(hidden)]
#[allow(non_upper_case_globals)]
static NUMBER: NUMBER = NUMBER { __private_field: () };
impl ::lazy_static::__Deref for NUMBER {
    type Target = u32;
    fn deref(&self) -> &u32 {
        #[inline(always)]
        fn __static_ref_initialize() -> u32 {
            21
        }
        #[inline(always)]
        fn __stability() -> &'static u32 {
            static LAZY: ::lazy_static::lazy::Lazy<u32> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for NUMBER {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
```

As you can see, the `NUMBER` variable is a struct used as an assisstant struct to implmenet the `::lazy_static::__Deref` and `::lazy_static::LazyStatic` traits.

- The `::lazy_static::__Deref` trait is used to implement the `deref` method for the `NUMBER` struct.
- The `::lazy_static::LazyStatic` trait is used to implement the `initialize` method for the `NUMBER` struct.

Check the `lazy_static` [repository](https://github.com/rust-lang-nursery/lazy-static.rs) for more details.
