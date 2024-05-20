mod commands;

use clap::Parser;

fn main() {
    let cli = commands::CliOpts::parse();

    if cli.verbose {
        println!("verbose: {}", cli.verbose);
    }

    commands::exec_command(cli.command);
}
