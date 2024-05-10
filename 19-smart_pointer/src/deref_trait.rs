use std::ops::Deref;

// A struct with one non-named T field.
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref trait.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {name}.");
}

pub fn test_deref() {
    println!("--- Start module: {}", module_path!());

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let b = Box::new(x);
    let mb = MyBox::new(x);
    
    assert_eq!(5, *b); // Box<T> implements Deref trait.    
    assert_eq!(5, *mb); // MyBox<T> implements Deref trait.

    let mbs = MyBox::new(String::from("Rust"));
    hello(&(*mbs)[..]); // Manually do the `&String -> &str` conversion.
    hello(&mbs);// Dereferences as `MyBox<String> -> &String -> &str`.

    println!("--- End module: {}", module_path!());
}
