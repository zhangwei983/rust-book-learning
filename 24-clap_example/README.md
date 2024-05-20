An example to how to use clap library to parse command line arguments.

## The clap library

The [clap](https://github.com/clap-rs/clap) library is a command line argument parser for Rust. Please check the [document](https://docs.rs/clap/latest/clap/) for how to use it.

### Installation

You can run

```bash
cargo add clap --features derive
```

to add clap dependency to the cargo.toml file in your project.

Please check this [cargo.toml](Cargo.toml) as an example.

### Using clap Parser

Please check [this code](./src/main.rs) as an example.

And you can run below command to see how to use this command line example.

```bash
cargo run -- -h
```

With `--`, you can pass arguments to the command line tool rather than `cargo run` itself.
