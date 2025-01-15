use strum::EnumIs;

#[allow(dead_code)]
#[derive(Debug, PartialEq, EnumIs)]
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
    println!("color_red is red: {}", color_red.is_red());
    let color_green = Color::Green { range: 10 };
    println!("color_green is green: {}", color_green.is_green());
    let color_blue = Color::Blue(10);
    println!("color_blue is blue: {}", color_blue.is_blue());
    println!("--- End module: {}", module_path!());
}
