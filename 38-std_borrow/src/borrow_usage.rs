use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    let s: &str = s.borrow();
    assert_eq!("Hello", s);
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let s = "Hello".to_string();
    check(s);
    //println!("{}", s); // Won't compile as s is moved to check.

    let s = "Hello";
    check(s);
    println!("{}", s);

    // This won't compile as i32 does not implement Borrow<str>.
    // let s = 1;
    // check(s);

    println!("--- End module: {}", module_path!());
}
