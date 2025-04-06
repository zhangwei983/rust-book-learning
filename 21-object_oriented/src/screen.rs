use crate::gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// Implement `Draw` trait defined in another module.
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Draw SelectBox: width {}, height {}, options {:?}",
            self.width, self.height, self.options
        );
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    println!("--- End module: {}", module_path!());
}
