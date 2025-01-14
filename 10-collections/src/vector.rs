use std::mem;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut v: Vec<i32> = Vec::new();

    // Rust will infer type of _v from the initial values.
    let _v = vec![1, 2, 3];

    // Modify vector.
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    // Index a value.
    let third = &v[2];
    println!("The third value is: {}", third);

    // Vec.get() returns an Option.
    let third = v.get(2);
    match third {
        Some(value) => println!("The third value is: {}", value),
        None => println!("There is no third value."),
    }

    // This will get None.
    // let third = &v[10]; // This will panic due to index out of range.
    let third = v.get(10);
    match third {
        Some(value) => println!("The third value is: {}", value),
        None => println!("There is no third value."),
    }

    let first = &v[0];
    // v.push(6); // This won't compile as the mutable borrow happens while an immutable borrow still exists.
    println!("The first element is {}", first);

    // Iterate values.
    for i in &v {
        println!("{i}");
    }

    // Iterate mutable values.
    for i in &mut v {
        *i *= 2;
    }

    for i in &v {
        println!("{i}");
    }

    // Vector size.
    let size = mem::size_of::<Vec<i32>>();
    println!("{size}");

    // Allow enum with different types. This allow Vector store different types.
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &_row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }

    println!("--- End module: {}", module_path!());
}
