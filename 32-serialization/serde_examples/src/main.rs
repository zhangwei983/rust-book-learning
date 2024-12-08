mod attributes;
mod basic;
mod default_values;

fn main() {
    basic::test();
    println!("");
    attributes::test();
    println!("");
    default_values::test();
}
