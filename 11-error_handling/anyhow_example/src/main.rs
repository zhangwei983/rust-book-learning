mod custom_error;
mod error_downcast;
mod existing_error;

fn main() {
    existing_error::test();
    println!("");
    custom_error::test();
    println!("");
    error_downcast::test();
}
