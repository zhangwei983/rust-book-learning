use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn test_rc_t() {
    println!("--- Start module: {}", module_path!());

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after a:  {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Count after b:  {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Count after c:  {}", Rc::strong_count(&a));
    }

    println!("Count after c out of scope:  {}", Rc::strong_count(&a));

    println!("--- End module: {}", module_path!());
}
