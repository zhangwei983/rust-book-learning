mod advanced_traits;
mod unsafe_rust;

fn main() {
    unsafe_rust::test_unsafe_rust();
    println!("");
    advanced_traits::test_advanced_traits();
}
