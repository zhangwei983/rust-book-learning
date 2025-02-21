use std::borrow::Cow;

trait IsBorrowed {
    fn my_is_borrowed(&self) -> bool;
}

impl IsBorrowed for Cow<'_, [i32]> {
    fn my_is_borrowed(&self) -> bool {
        match self {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false,
        }
    }
}

impl IsBorrowed for Cow<'_, str> {
    fn my_is_borrowed(&self) -> bool {
        match self {
            Cow::Borrowed(_) => true,
            Cow::Owned(_) => false,
        }
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
    println!("First: {}", input.my_is_borrowed());
    abs_all(&mut input);
    println!("First: {}", input.my_is_borrowed());

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    println!("Second: {}", input.my_is_borrowed());
    abs_all(&mut input);
    println!("Second: {}", input.my_is_borrowed());

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    println!("Third: {}", input.my_is_borrowed());
    abs_all(&mut input);
    println!("Third: {}", input.my_is_borrowed());

    for number in 1..=6 {
        println!("{} : {}", number, modulo_3(number).my_is_borrowed());
    }

    println!("--- End module: {}", module_path!());
}
