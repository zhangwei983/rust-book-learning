use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Failed parsing arguments: {error}");
        process::exit(1);
    });

    // println!("Search for '{}'", config.query);
    // println!("In file '{}'", config.file_path);

    if let Err(err) = minigrep::run(config) { // Here the config will be moved.
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
