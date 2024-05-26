use macros::{add_as, caculate, my_vec};

#[derive(Debug)]
struct TestMacro {}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("My vec: {:?}", v);

    let a = add_as!(1, 2, 3, 4);
    println!("Add as: {}", a);

    caculate! (
        (1 + 2) * (3 + 4)
    )
}
