mod borrow_mut_usage;
mod borrow_usage;

fn main() {
    borrow_usage::test();
    println!("");
    borrow_mut_usage::test();
}
