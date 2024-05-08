An example to show the module system in Rust.

The code in this example is from [Clear explanation of Rust’s module system](https://www.sheshbabu.com/posts/rust-module-system/).

## What is Crate

A crate is the smallest amount of code that the Rust compiler considers at a time.

Crates can contain `modules`, and the modules may be defined in other files that get compiled with the crate.

A crate can come in one of two forms: a `binary` crate or a `library` crate. 

- `Binary` crates are programs you can compile to an `executable` that you can run. Each must have a function called `main` that defines what happens when the executable runs. 

- `Library` crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be `shared` with multiple projects.

Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".

The crate `root` is a source file that the Rust compiler starts from and makes up the `root module` of your crate.

## What is Package

A package is a `bundle` of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

A package can contain as many binary crates as you like, but at most only one library crate.

A package must contain `at least` one crate, whether that’s a library or binary crate.

The below is the [Cargo.toml](./Cargo.toml) file of this package.

```rust
[package]
name = "module_system"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

It shows:
- the name of the package, as `module_system` in this case.
- no `src/main.rs` mentioned.  
   Cargo follows a convention that `src/main.rs` is the crate `root` of a `binary` crate with the `same` name as the package.

## Modules Cheat Sheet

A quick reference on how `modules`, `paths`, the `use` keyword, and the `pub` keyword work in the compiler, and how most developers organize their code. 

### Start from the Crate Root

When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

### Declaring Modules

In the crate root file, you can declare new modules. For example, you declare a `garden` module with `mod garden;`, the compiler will look for the module’s code in these places:

- inline, within `curly brackets` that replace the semicolon following mod garden
- in the file `src/garden.rs`
- in the file `src/garden/mod.rs`

### Declaring Submodules

In any file other than the crate root, you can declare submodules. For example, you declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:

- inline, directly following mod vegetables, within curly brackets instead of the semicolon
- in the file `src/garden/vegetables.rs`
- in the file `src/garden/vegetables/mod.rs`

### Paths to code in modules

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. 

For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

### Private vs Public

Code within a module is `private` from its `parent` modules by default. 

- To make a module public, declare it with pub mod instead of mod. 
- To make items within a public module public as well, use pub before their declarations.

### The `use` Keyword

Within a scope, the `use` keyword creates `shortcuts` to items to reduce repetition of long paths.

For example, you can create a `shortcut` with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.
