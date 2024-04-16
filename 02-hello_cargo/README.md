A example of cargo usage on Rust project. Cargo book can be found at https://doc.rust-lang.org/cargo/.

With simple projects, Cargo doesn’t provide a lot of value over just using `rustc`, but it will prove its worth as your programs become more intricate. Once programs grow to multiple files or need a dependency, it’s much easier to let Cargo coordinate the build.

Use the below command to create the project 
```
cargo new 02-hello_cargo --name hello_cargo
cd 02-hello_cargo
```

Things to notic:
- `cargo new` has generated a "Hello, world!" program.
- Check `cargo.toml` file, https://toml.io/en/.
- The reason for `--name` is the package name cannot start with digital number `02`. I have to use `--name` to set a different package name to the directory name. 

How to run:
1. cargo build
  - Creates an executable file in `target/debug`.
  - `-- release` argument for release build, in `target/release`.

2. cargo run
  Build and run a project in one step.

3. cargo check
  Build a project without producing a binary.
