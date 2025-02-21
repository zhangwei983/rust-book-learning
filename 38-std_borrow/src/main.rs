mod borrow_mut_usage;
mod borrow_usage;
mod cow_usage;
mod to_owned_usage;

fn main() {
    borrow_usage::test();
    println!("");
    borrow_mut_usage::test();
    println!("");
    to_owned_usage::test();
    println!("");
    cow_usage::test();
}
