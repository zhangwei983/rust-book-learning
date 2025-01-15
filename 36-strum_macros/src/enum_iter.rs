use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator};

#[allow(dead_code)]
#[derive(Debug, PartialEq, EnumCount, EnumIter, EnumString)]
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
    println!("The number of colors is {}", Color::COUNT);

    for color in Color::iter() {
        println!("{:?}", color);
    }
    println!("--- End module: {}", module_path!());
}
