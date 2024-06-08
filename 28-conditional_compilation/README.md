An example to show the conditional compilation in Rust.

## Conditional Compilation

Conditional compliation is taking some source code into compilation depending on certain conditions.

In Rust, source code can be conditionally compiled using the 
- `cfg` attribute,
- `cfg_attr` attribute,
- and the built-in `cfg` macro.

Each form of conditional compilation takes a configuration predicate that evaluates to true or false. The predicate is one of the following:

- `A configuration option`.  
  It is true if the option is set and false if it is unset.
- `all()` with a comma separated list of configuration predicates.  
  It is false if at least one predicate is false. If there are no predicates, it is true.
- `any()` with a comma separated list of configuration predicates.  
  It is true if at least one predicate is true. If there are no predicates, it is false.
- `not()` with a configuration predicate.  
  It is true if its predicate is false and false if its predicate is true.


## The `cfg` Attribute

The `cfg` attribute conditionally includes the thing it is attached to based on a configuration predicate.


You can use the cfg attribute `#[cfg(...)]` in attribute position as below.

```rust
// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}

// This function is only included when either foo or bar is defined
#[cfg(any(foo, bar))]
fn needs_foo_or_bar() {
  // ...
}

// This function is only included when compiling for a unixish OS with a 32-bit
// architecture
#[cfg(all(unix, target_pointer_width = "32"))]
fn on_32bit_unix() {
  // ...
}

// This function is only included when foo is not defined
#[cfg(not(foo))]
fn needs_not_foo() {
  // ...
}

// This function is only included when the panic strategy is set to unwind
#[cfg(panic = "unwind")]
fn when_unwinding() {
  // ...
}
```

## The `cfg` Macro

The built-in `cfg` macro takes in a single configuration predicate and evaluates to the true literal when the predicate is true and the false literal when it is false.

You can use `cfg!(...)` in boolean expressions as below.

```rust
let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);
```

## Configuration Conditionals

You can set conditionals including:
- target_arch
- target_feature
- target_os
- target_family
- target_env
- target_abi
- target_endian
- target_pointer_width
- target_vendor
- target_has_atomic
- ...

You can check [this document](https://doc.rust-lang.org/stable/reference/conditional-compilation.html#set-configuration-options) for the listed available values for the above options. The lists may not be a fully set. For example, `wasm32` can be a value for `target_arch` which is not listed on the above link.

> [!NOTE]  
> The `unix` and `windows` options are special cases for `target_family = "unix"` and `target_family = "windows"`.

## Custom Conditionals 

You can define custome conditionals like 

```rust
#[cfg(test_condition)]
fn conditional_function() {
    println!("Test condition met!");
}
```

But you need to pass the customized conditionals to `rustc` with `--cfg` flag. You can also define build flags in the cargo config.toml file.

- Create your `.cargo/config.toml` file, check the [cargo configuration file](https://doc.rust-lang.org/stable/cargo/reference/config.html#hierarchical-structure) for details.
- Add your own flag to the `[build]` table.

Here is an example of a [.cargo/config.toml](./.cargo/config.toml), which contains the below configuration

```
[build]
rustflags = ["--cfg", "test_condition", "--cfg", "test_condition_1"]
```
to pass two custom conditionals `test_condition` and `test_condition_1` to `rustc`.

With this, the `conditional_function` and `conditional_function_1` functions are visible in [cfg_attribute.rs](./src/cfg_attribute.rs#L28) for the current build.
