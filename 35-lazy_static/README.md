An example to show the basic usage of the [lazy_static crate](https://crates.io/crates/lazy_static).

'lazy_static' is a macro that allows you to define static variables that are initialized lazily. 

The variables are 
- initialized when they are first accessed, 
- and the initialization code is only run once. 
- immutable, but if you need to modify them, you can use a Mutex to make them mutable.
