use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // &'static str is a shared reference to a string, and the lifetime is the lifetime of the program.
    // The MAP is immutable.
    static ref MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "one");
        m.insert(2, "two");
        m
    };

    // The MUTABLE_MAP is mutable.
    static ref MUTABLE_MAP: Mutex<HashMap<u32, &'static str>> = {
        let mut m = HashMap::new();
        m.insert(1, "one");
        m.insert(2, "two");
        Mutex::new(m)
    };
    static ref COUNT: usize = MUTABLE_MAP.lock().unwrap().len();

    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 {
    n * 2
}

fn main() {
    println!("The map entry '0' is {}", *MAP.get(&1).unwrap());
    // MAP.insert(3, "three"); This is not allowed because MAP is immutable.

    println!("The COUNT is {}", *COUNT);
    println!(
        "The mutalbe map entry count is {}",
        MUTABLE_MAP.lock().unwrap().len()
    );
    MUTABLE_MAP.lock().unwrap().insert(3, "three");
    println!("The COUNT is still {}", *COUNT); // The COUNT won't change.
    println!(
        "The mutalbe map entry count is {}",
        MUTABLE_MAP.lock().unwrap().len()
    );

    println!("The number is {}", *NUMBER);
}
