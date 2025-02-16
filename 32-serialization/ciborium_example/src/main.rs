use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    let file = File::create("point.cbor").unwrap();
    ciborium::ser::into_writer(&point, file).unwrap();

    let file = File::open("point.cbor").unwrap();
    let result: Point = ciborium::de::from_reader(file).unwrap();
    assert_eq!(point, result);
    println!("{:?}", result);

    let mut bytes = vec![];
    ciborium::ser::into_writer(&point, &mut bytes).unwrap();

    let result: Point = ciborium::de::from_reader(bytes.as_slice()).unwrap();
    assert_eq!(point, result);
    println!("{:?}", result);
}
