mod custom_error;
mod existing_error;

fn main() {
    existing_error::test();
    println!("");
    custom_error::test();
}
