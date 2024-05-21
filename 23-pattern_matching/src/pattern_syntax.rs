fn test_basic_match() {
    // Match literals.
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }

    // Match named variables.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Fifty"),
        Some(y) => println!("x = {}", y), // This y hides the `let y`, and match x Some(5).
        _ => println!("Other"),
    }
    println!("x = {:?}, y = {y}", x);

    // Match multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        _ => println!("Other"),
    }

    // Match range.
    let x = 5;
    match x {
        1..=5 => println!("One to five"),
        _ => println!("Other"),  
    }

    let ch = 'c';
    match ch {
        'a'..='j' => println!("a to j"),
        'k'..='z' => println!("k to z"),
        _ => println!("Other"),
    }
}

fn test_destructure_structs() {
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };

    // Creates the variables `a` and `b` that match the values of the x and y fields of the point struct.
    let Point { x: a, y: b } = point;
    println!("a: {}, b: {}", a, b);

    // A shorter version.
    let Point { x, y } = point;
    println!("x: {}, y: {}", x, y);

    // Destructure with partial literal values.
    match point {
        Point { x, y: 0 } => println!("On X axis: ({}, {})", x, y), // y has to match 0.
        Point { x: 0, y} => println!("On Y axis: ({}, {})", x, y), // x has to match 0.
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructure structs and tuples.
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    assert_eq!(feet, 3);
    assert_eq!(inches, 10);
    assert_eq!(x, 3);
    assert_eq!(y, -10);
}

fn test_destructure_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum Message {
        Quit,                       // Contains no data.
        Move { x: i32, y: i32 },    // Contains a struct
        Write(String),              // Contain a tuple.
        ChangeColor(Color),         // Contain a tuple.
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {x} and in the y direction {y}"),
        Message::Write (str) => println!("Text message: {str}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change the color to r {r}, g {g}, and b {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change the color to h {}, s {}, v {}", h, s, v),
    }
}

// Use the underscore as a wildcard pattern that will match any value but not bind to the value.
fn foo(_: i32, y: i32) {
    println!("Only uses the y parameter: {}", y);
}

fn test_ignore_values() {
    foo(3, 4);

    let mut value = Some(5);
    let new_value = Some(10);

    // Ingore partially.
    match (value, new_value) {
        (Some(_), _) => {
            println!("Can't overwrite an existing value");
        }
        _ => {
            value = new_value;
        }
    }

    println!("Value is {:?}", value);

    // Ignore partially from a tuple.
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("{}, {}, {}", first, third, fifth);
        }
    }

    // Ignore a range part with `..`.
    match numbers {
        (first, .., last) => {
            println!("First is: {}, last is: {}", first, last);
        }
    }
}

fn test_match_guards() {
    let number = Some(4);

    // Use additional if as a match guard.
    match number {
        Some(x) if x % 2 == 0 => println!("{} is even", x),
        Some(x) => println!("{} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    // Use additional if as a match guard.
    match x {
        Some(n) if n == y => println!("x matched y"),
        _ => println!("x didn't match y"),
    }
}

fn test_match_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // Binding id to id_var only if id is in range `3..=7`.
        Message::Hello {
            id: id_var @ 3..=7
        } => println!("Id in range 3-7: {}", id_var),
        // Only check if id is in range `8..=10` without binding it to a variable, so we cannot use it's value.
        Message::Hello { id: 8..=10 } => println!("Id in range 8-10."),
        Message::Hello { id } => println!("Id in other range: {}", id),
    }
}

pub fn test_pattern_syntax() {
    println!("--- Start module: {}", module_path!());

    test_basic_match();

    test_destructure_structs();

    test_destructure_enums();

    test_ignore_values();

    test_match_guards();

    test_match_bindings();

    println!("--- End module: {}", module_path!());
}
