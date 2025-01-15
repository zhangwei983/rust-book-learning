use std::string::ToString;
use strum::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Display)]
enum Color {
    Red,
    #[strum(to_string = "Green with range {range}")]
    Green {
        range: usize,
    },
    #[strum(to_string = "Blue with value {0}")]
    Blue(usize),
    #[strum(disabled)]
    Yellow,
    #[strum(ascii_case_insensitive)]
    Black,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());
    let color_red = Color::Red;
    println!("{}", color_red.to_string());
    let color_green = Color::Green { range: 10 };
    println!("{}", color_green.to_string());
    let color_blue = Color::Blue(10);
    println!("{}", color_blue.to_string());
    println!("--- End module: {}", module_path!());
}
