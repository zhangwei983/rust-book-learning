use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn test_ref_cycle() {
    println!("--- Start module: {}", module_path!());

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc count is: {}", Rc::strong_count(&a));
    println!("a next item is: {:?}", a.next());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count is: {}", Rc::strong_count(&a));
    println!("b rc count is: {}", Rc::strong_count(&b));
    println!("b next time is: {:?}", b.next());

    // Construct a ref cycle: a->b->a->b...
    if let Some(link) = a.next() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a rc count is: {}", Rc::strong_count(&a));
    println!("b rc count is: {}", Rc::strong_count(&b));

    // This will overflow the stack.
    // println!("a next item = {:?}", a.next());

    println!("--- End module: {}", module_path!());
}
