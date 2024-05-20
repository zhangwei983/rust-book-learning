pub fn test_pattern_matching() {
    println!("--- Start module: {}", module_path!());

    let x = Some(5);

    // Basic pattern matching arms.
    match x {
        Some(i) => { 
            println!("x: {}", i);
        }
        None => (),
    };

    // Use `if let` to match for one case.
    if let Some(i) = x {
        println!("x: {}", i);
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Use `while let` to loop as long as the pattern continues to match.
    while let Some(i) = stack.pop() {
        println!("Top: {}", i);
    }

    let v = vec!['a', 'b', 'c'];

    // Match the tuple produced by the iterator returned by enumerate() method.
    for (index, value) in v.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }

    // This is also pattern matching!
    // let PATTERN = EXPRESSION;
    let _x = 5;
    let (_x, _y) = (1, 2);
    let (_x, _y, _z) = (1, 2, 3);
    // let (_x, _y) = (1, 2, 3); // This won't compile as the tuple doesn't mattch the number of elements.

    // The function parameter is also a pattern.
    let point = (3, 5);
    print_coordinates(&point);

    println!("--- End module: {}", module_path!());
}

fn print_coordinates(&(x, y) : &(i32, i32)) {
    println!("Coordinates: {}, {}", x, y);
}