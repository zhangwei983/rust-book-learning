use anyhow::{Context, Result};

fn main() {
    match read_config_file("Hello.txt") {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{:?}", err),
    }
}

fn read_config_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read the file at path: {}", path)) // Add more context to the error.
}
