use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Closure definitions.
    fn  _add_one_v1 (x: u32) -> u32 { x + 1 }                           // Defined a function.
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };   // Defined a closure.
    // let _add_one_v3 = |x| { x + 1 };                                 // Won't compile due to lack of type.
    // let _add_one_v4 = |x| x + 1 ;                                    // Won't compile due to lack of type.

    // Type inferring at compiling time.
    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    // let n = example_closure(5); // This won't compile as the type of `x` has been inferred as String.

    println!("example_closure returns :{s}");

    // Capture references.
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // The variable `list `is bound to a closure definition.
    let immutable_borrow = || println!("From immutable closure: {:?}", list);

    println!("Before calling immutable closure: {:?}", list); // This is fine, as the `only_borrows` closure only defines immutable borrow.
    immutable_borrow();
    println!("After calling immutable closure: {:?}", list);

    // When mutable_borrow is defined, it captures a mutable reference to list. 
    let mut mutable_borrow = || list.push(7);

    // println!("Before calling mutable closure: {:?}", list); // This won't compile as mutable borrow happens later.
    mutable_borrow();
    println!("After calling mutable closure: {:?}", list);

    // move is required here.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


    // Usage of the Fn Traits.
    let mut rect_list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Use sort_by_key to order the list by the width attribute from low to high.
    rect_list.sort_by_key(|r| r.width);
    println!("{:#?}", rect_list);

    /* This won't compile as the `value` will be moved out of the the closure.
    let mut sort_operations = vec![];
    let value = String::from("by key called");

    rect_list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
     */

     // Implement a FnMut closure.
     let mut num_sort_operations = 0;
     rect_list.sort_by_key(|r| {
         num_sort_operations += 1;
         r.width
     });

    println!("{:#?}", rect_list);
    println!("{:#?}, sorted in {num_sort_operations} operations", rect_list);
}
