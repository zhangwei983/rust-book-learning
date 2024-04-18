fn main() {
    // Simple usage of String
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    // Copy
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);

    // Move
    let s1 = String::from("hello");
    // let s2 = s1; // This is where Move happens, and will introduce compiling errors in next line where s1 is borrowed.
    println!("{}", s1); 

    let mut s2 = s1.clone(); // clone will do Deep Copy, both str1 and str2 are valid.
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    takes_ownership(s1); // str1 moves into takes_ownership(), and is invalidated.

    // println!("{}", s1); // This will introduce compiling error, as str1 is invalid.

    s2 = takes_and_gives_ownership(s2);

    println!("{}", s2); // It's okay to print str2 as ownership is given back to it.

    // References and borrowing
    let len = get_length(&s2);
    println!("str2 length is {len}");

    // Mutable references.
    change_str(&mut s2);
    println!("Changed str2 is: {s2}");

    let ref1 = &mut s2;
    // let ref2 = &mut s2; // This will get compiling error as only one mutable reference is allowed.
    // let ref2 = &s2; // This will get compiling error as the scopes of mutable and immutable references cannot overlap.

    println!("Mutable refenence: {}", ref1);

    let first = first_word(&s2); // &String is implicitly converted to &str by Deref Coercions.

    // s2.clear(); // This will get compiling error as s2 is referenced by the String slice.

    println!("First word is: {}", first);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called, the memory on heap is freed.

fn takes_and_gives_ownership(some_string: String) -> String { // some_string comes into scope
    some_string
} // some_string goes out of scope and `drop` is called, the memory on heap is freed.

fn get_length(s: &String) -> usize {
    s.len()
} // str goes out of scope, but the underneath string content won't be freed as it does not have the ownership.

fn change_str(s: &mut String) {
    s.push_str(" world")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
