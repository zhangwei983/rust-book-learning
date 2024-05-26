use macros::{add_as, caculate, my_vec};
use macros::HelloMacro;
use macro_derive::HelloMacro;

// Derive macro.
#[derive(HelloMacro)]
struct Pancakes  {}

fn main() {
    // // Declarative macros
    let v = my_vec![1, 2, 3];
    println!("My vec: {:?}", v);

    let a = add_as!(1, 2, 3, 4);
    println!("Add as: {}", a);

    caculate! (
        (1 + 2) * (3 + 4)
    );

    // Derive macro.
    Pancakes::hello_macro();
}
