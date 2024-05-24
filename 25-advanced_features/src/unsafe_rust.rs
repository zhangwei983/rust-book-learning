use std::slice;

// Define an unsafe function.
unsafe fn dangerous() {}

// Use unsafe code to split a slice.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..]) // Can't compile due to multiple mutable borrows.

    let ptr = values.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Set up an integration with the abs function from the C standard library.
extern "C" {
    fn abs(input: i32) -> i32;
}


// Static immutable variable.
static HELLO_WORLD: &str = "Hello world";

// Static mutable variable.
static mut COUNTER: u32 = 0;

fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe trait.
unsafe trait Foo {}

unsafe impl Foo for i32 {}

pub fn test_unsafe_rust() {
    println!("--- Start module: {}", module_path!());

    let mut num = 5;

    // Use `as`` to cast references into raw pointer types. 
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // This is an invalid raw pointer.
    let address = 0x123456usize;
    let _r = address as *const i32; 

    unsafe {
        // println!("r is: {}", *_r); // Dereference an invalid raw pointer will crash the program.
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Update the value via a mutable pointer
        *r2 = 10;
        println!("r2 is: {}", *r2);
    }

    // Call unsafe function in a unsafe block.
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 2);
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // Extern function call in an unsafe block.
    unsafe {
        println!("Abs of -3: {}", abs(-3));
    }

    println!("Immutable static: {}", HELLO_WORLD);

    add_counter(2);

    // Access mutable static variables in an unsafe block.
    unsafe {
        println!("Mutable static: {}", COUNTER);
    }

    println!("--- End module: {}", module_path!());
}
