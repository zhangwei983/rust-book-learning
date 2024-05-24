mod advanced_traits;
mod advanced_types;
mod unsafe_rust;

fn main() {
    unsafe_rust::test_unsafe_rust();
    println!("");
    advanced_traits::test_advanced_traits();
    println!("");
    advanced_types::test_advanced_types();
}
