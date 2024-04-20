// Generic struct.
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x (&self) -> &T {
        &self.x
    }
}

// Only implement for type f32.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let list = vec![10, 20, 50, 40, 25];

    let largest = find_largest_i32(&list);

    println!("Largest number in the list is: {largest}");

    // Generic in function.
    let generic_largest = find_largest(&list);
    println!("Largest number in the list is: {generic_largest}");

    // Generic in struct.
    let integer = Point { x: 5, y: 10};
    let float = Point { x: 1.0f32, y: 2.0};

    println!("Integer x is {}", integer.x());
    println!("Float x is {}", float.x());

    // println!("Integer distance is {}", integer.distance_from_origin()); // This won't compile as distance_from_origin is only implemented for f32.
    println!("Float distance is {}", float.distance_from_origin());
}

// No generic version.
fn find_largest_i32 (list: &[i32]) -> &i32 { // &[i32] is a type of slice.
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// Generic version.
fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {  // Restrict to types that implement `std::cmp::PartialOrd` trait.
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
