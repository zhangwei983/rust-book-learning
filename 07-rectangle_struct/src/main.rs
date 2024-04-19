#[derive(Debug)] // Enable print.
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement the functions for Rectangle struct.
impl Rectangle {
    fn area(&self) -> u32 {
    // fn area(self: &Rectangle) -> u32 { // This is right too.
        self.width * self.height
    }
}

// Multiple impl blocks is supported.
impl Rectangle {
    fn square(size: u32) -> Self { // An function associated to the Type, not the instance.
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("Rectangle is: {:?}", rect1); // The specifier `:?` tells `println!` to use `Debug` output format.
    println!("Rectangle is: {:#?}", rect1); // The specifier `{:#?}` has better output format.

    dbg!(&rect1); // Another way to use Debug format.

    println!("Rectangle area is: {}", rect1.area());
    println!("Rectangle area is: {}", area(&rect1));

    let square1 = Rectangle::square(30);

    println!("Square is: {:?}", square1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
