use std::convert::AsRef;
use strum::AsRefStr;

#[allow(dead_code)]
#[derive(Debug, AsRefStr)]
#[strum(prefix = "Prefix: ")]
enum Color {
    #[strum(serialize = "redred")]
    Red,
    #[strum(to_string = "Green with range {range}")]
    Green { range: usize },
    #[strum(to_string = "Blue with value {0}")]
    Blue(usize),
}

pub fn test() {
    println!("--- Start module: {}", module_path!());
    let color_red = Color::Red;
    println!("{}", color_red.as_ref());
    let color_green = Color::Green { range: 10 };
    println!("{}", color_green.as_ref());
    let color_blue = Color::Blue(10);
    println!("{}", color_blue.as_ref());

    println!("--- End module: {}", module_path!());
}
