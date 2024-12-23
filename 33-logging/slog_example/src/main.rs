mod log_to_file;
mod log_to_json;
mod log_to_multiple_drains;
mod log_to_terminal;
mod logging_level;

fn main() {
    log_to_terminal::test();
    println!("");
    logging_level::test();
    println!("");
    log_to_file::test();
    println!("");
    log_to_multiple_drains::test();
    println!("");
    log_to_json::test();
}
