mod attributes;
mod basic;
mod custom_deserialize;
mod custom_serialize;
mod default_values;
mod flattern;
mod process_array;

fn main() {
    basic::test();
    println!("");
    attributes::test();
    println!("");
    default_values::test();
    println!("");
    flattern::test();
    println!("");
    custom_serialize::test();
    println!("");
    custom_deserialize::test();
    println!("");
    process_array::test();
}
