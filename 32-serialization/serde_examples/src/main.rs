mod attributes;
mod basic;
mod default_values;
mod flattern;

fn main() {
    basic::test();
    println!("");
    attributes::test();
    println!("");
    default_values::test();
    println!("");
    flattern::test();
}
