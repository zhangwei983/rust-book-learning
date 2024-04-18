fn main() {
    // Usage of mut.
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Usage of Shadowing.
    let y = 5;
    println!("The value of y is: {y}");
    
    let y = y + 1;
    println!("The value of y is: {y}");

    {
        let y = y * 2;
        println!("The value of y is: {y}");
    }

    println!("The value of y is: {y}");

    // Usage of tuples.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("First is: {}", tup.0);

    let (x, _y, _z) = tup;
    println!("First is: {x}");

    // Usage of arrays.
    let array0 = [3; 5]; // [N; X]
    println!("The first element is {}", array0[0]);
    println!("Array is: {:?}", array0);

    // Function.
    function_no_parameter();

    let mut z = 5;
    z = function_with_parameters(z);
    println!("{z}");

    // If control flow.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!(" Number value is: {number}");

    // Loop label
    let mut count = 0;
    'outer_loop: loop {
        println!("Entered the outer loop");

        let mut remaining = 10;
        loop {
            println!("Entered the inner loop");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'outer_loop;
            }

            remaining -= 1;
        }
        
        count += 1;
    }

    println!("Count is: {count}");

    // for loop
    let array1 = [10, 20, 30, 40, 50, 60];

    for element in array1 {
        println!("{element}");
    }

    // range loop
    for element in 1..5 {
        println!("{element}");
    }

    for element in (1..=5).rev() {
        println!("{element}");
    }
}

fn function_no_parameter() {
    println!("From function no parameter.")
}

fn function_with_parameters(x: i32) -> i32 {
    println!("From function with parameters.");
    x + 1
}
