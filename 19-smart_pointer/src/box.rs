#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn test_box_t() {
    println!("--- Start module: {}", module_path!());

    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    match list {
        Cons(i, value) => {
            println!("{}, {:?}", i, value);
        }
        Nil => ()
    }

    println!("--- End module: {}", module_path!());
}
