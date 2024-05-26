use macros::{add_as, caculate, my_vec};
use macros::HelloMacro;
use macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes  {}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("My vec: {:?}", v);

    let a = add_as!(1, 2, 3, 4);
    println!("Add as: {}", a);

    caculate! (
        (1 + 2) * (3 + 4)
    );

    Pancakes::hello_macro();
}
