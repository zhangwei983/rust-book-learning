use std::borrow::Cow;

fn is_borrowed(input: &Cow<'_, [i32]>) -> bool {
    match input {
        Cow::Borrowed(_) => true,
        Cow::Owned(_) => false,
    }
}

fn abs_all(input: &mut Cow<'_, [i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(), // This implements as Cow::Borrowed(s) for str.
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(), // This implements as Cow::Owned(s) for String.
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let slice = [1, 2, 3];
    let mut input = Cow::from(&slice[..]);
    println!("First: {}", is_borrowed(&input));
    abs_all(&mut input);
    println!("First: {}", is_borrowed(&input));

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    println!("Second: {}", is_borrowed(&input));
    abs_all(&mut input);
    println!("Second: {}", is_borrowed(&input));

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    println!("Third: {}", is_borrowed(&input));
    abs_all(&mut input);
    println!("Third: {}", is_borrowed(&input));

    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(msg) => println!("{} : The Cow is borrowed with : {}", number, msg),
            Cow::Owned(msg) => println!("{} : The Cow is owned with : {}", number, msg),
        }
    }

    println!("--- End module: {}", module_path!());
}
