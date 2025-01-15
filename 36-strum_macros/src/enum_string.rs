use std::str::FromStr;
use strum::EnumString;

#[allow(dead_code)]
#[derive(Debug, PartialEq, EnumString)]
enum Color {
    Red,
    Green {
        range: usize,
    },
    #[strum(serialize = "blue", serialize = "b")]
    Blue(usize),
    #[strum(disabled)]
    Yellow,
    #[strum(ascii_case_insensitive)]
    Black,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());
    let color_red = Color::from_str("Red").unwrap();
    println!("{:?}", color_red);
    let color_green = Color::from_str("Green").unwrap();
    println!("{:?}", color_green);
    let color_blue = Color::from_str("blue").unwrap();
    println!("{:?}", color_blue);
    let color_blue = Color::from_str("b").unwrap();
    println!("{:?}", color_blue);
    let color_black = Color::from_str("Black").unwrap();
    println!("{:?}", color_black);

    // This will return an error as Yellow is disabled.
    let color_yellow = Color::from_str("Yellow");
    println!("{:?}", color_yellow);
    println!("--- End module: {}", module_path!());
}
