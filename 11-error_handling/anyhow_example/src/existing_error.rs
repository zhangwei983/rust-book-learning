use anyhow::{Context, Result};

fn read_config_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read the file at path: {}", path)) // Add more context to the error.
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    match read_config_file("Hello.txt") {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{:?}", err),
    }

    println!("--- End module: {}", module_path!());
}
