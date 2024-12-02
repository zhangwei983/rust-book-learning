use clap::{Parser, Subcommand};

#[derive(Parser)] // Derive `Parser` to start building your own parser.
#[command(name = "MyApp")] // Override the app name in cargo.toml.
#[command(version = "1.0")] // Override the version in cargo.toml.
#[command(author = "Vincent")] // Set the author.
#[command(about = "Clap example", long_about = None)]
#[command(next_line_help = true)] // Print the help message in next line.
pub struct CliOpts {
    #[arg(short, long)]
    // A bool flag.
    pub verbose: bool,

    #[command(subcommand)]
    // A subcommand.
    pub command: Command,
}

#[derive(Parser)]
#[command(name = "greet")]
pub struct GreetOpts {
    #[arg(short, long)]
    // Have both the short and long format, e.g. `-n` for short, and `--name` for long.
    name: String,

    #[arg(short, long, default_value_t = 1)]
    // With default value `1`.
    count: u32,

    // Argument that is read from the environment variable, which will be hidden from the command line.
    #[arg(long, env = "TEST_ENVIRONMENT_VARIABLE", hide = true)]
    env_variable_set: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Greet to the name.
    Greet(GreetOpts),
}

pub fn exec_command(cmd: Command) {
    match cmd {
        Command::Greet(opts) => {
            for _ in 0..opts.count {
                println!("Hello {}.", opts.name);
                println!("{}", opts.env_variable_set);
            }
        }
    }
}
