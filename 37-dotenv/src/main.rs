use dotenv::dotenv;
use dotenv_codegen::dotenv;
use std::env;

fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or("Not provided".to_string());
    println!("LOG_LEVEL: {}", log_level);

    dotenv().ok();

    let log_level = env::var("LOG_LEVEL").unwrap_or("Not provided".to_string());
    println!("LOG_LEVEL: {}", log_level);

    // dotenv! brings the variables from the .env file at compile time.
    // This means you don't need the .env file or dotenv() at runtime.
    let log_level = dotenv!("LOG_LEVEL");
    println!("LOG_LEVEL: {}", log_level);
}
