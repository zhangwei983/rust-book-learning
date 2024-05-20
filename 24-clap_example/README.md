An example to how to use clap library to parse command line arguments.

## The clap library

The [clap](https://github.com/clap-rs/clap) library is a command line argument parser for Rust. Please check the [document](https://docs.rs/clap/latest/clap/) for how to use it.

### Installation

You can run

```bash
cargo add clap --features derive
```

to add clap dependency to the cargo.toml file in your project. Please check this [cargo.toml](Cargo.toml) as an example.

And for how to use `derive`, please check [this document](https://docs.rs/clap/4.0.18/clap/_derive/_tutorial/index.html#).

### Using clap Parser

[Here](./src/commands.rs) is an example of using clap to build a simple command line tool.

And you can run below command to see how to use this command line example.

```bash
cargo run -- -h
```

With `--`, you can pass arguments to the command line tool rather than `cargo run` itself.

So we can use the cli we build as

```bash
cargo run -- greet --name World --count 5
```

to greet to the name `World` for `5` times.
