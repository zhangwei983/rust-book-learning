use std::ops::Add;
use std::fmt;

// Define with associated types.
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Define with generics.
pub trait Iterator1<T> {
    fn next(&mut self) -> Option<T>;
}

// Implement with associated types.
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         ...
//     }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Implement Add trait with associated types.
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

// Implement Add traits with both generics and associated types.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Flying as a pilot.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Flying as a wizard.");
    }
}

impl Human {
    fn fly(&self) {
        println!("Flying as a human.");
    }
}

trait Animal {
    fn name() -> String;
}

struct Dog;

impl Animal for Dog {
    fn name() -> String {
        String::from("Puppy")
    }
}

impl Dog {
    fn name() -> String {
        String::from("Spot")
    }
}

// Define a trait that requires another trait.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }    
}

struct Point1 {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point1 {}

// Implement the `fmt::Display` trait required by `OutlinePrint` trait.
impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// New type to implement the external `fmt::Display` trait on the external type `Vec<String>`.
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn test_advanced_traits() {
    println!("--- Start module: {}", module_path!());

    let p0 = Point { x: 1, y: 1 };
    let p1 = Point { x: 2, y: 2 };

    println!("Added point: {:?}", p0 + p1);

    let mm = Millimeters(100u32);
    let m = Meters(1u32);

    println!("Add millimeters: {:?}", mm + m);

    let human = Human;
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);

    println!("Dog name: {}", Dog::name());
    // println!("Animal name: {}", Animal::name()); // Won't compile as Rust cannot figure out which type to use here.
    println!("Animal name: {}", <Dog as Animal>::name());

    let point1 = Point1 { x: 3, y: 5 };
    point1.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper: {}", w);

    println!("--- End module: {}", module_path!());
}
