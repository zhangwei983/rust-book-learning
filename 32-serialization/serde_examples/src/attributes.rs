use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    #[serde(rename = "x_name")]
    // Customized the serialization name for `x` field.
    x: i32,
    #[serde(rename(serialize = "y_name", deserialize = "y_name"))]
    // Customize the serialize/deserialize name separately, simply keep them same for the example.
    y: i32,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let point = Point { x: 1, y: 2 };

    let serialized_point = serde_json::to_string(&point).unwrap();
    println!("Serialized point: {}", serialized_point);

    let deserialzed_point: Point = serde_json::from_str(&serialized_point).unwrap();

    println!("Deserialized point: {:?}", deserialzed_point);

    println!("--- End module: {}", module_path!());
}
