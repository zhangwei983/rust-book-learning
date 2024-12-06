use serde::{Deserialize, Serialize};

// Example for a struct.
#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    names: Vec<String>,
    values: Vec<String>,
}

// Example for an Enum.
#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: String,
    },
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let point = Point { x: 1, y: 2 };

    let serialized_point = serde_json::to_string(&point).unwrap();
    println!("Serialized point: {}", serialized_point);

    let deserialzed_point: Point = serde_json::from_str(&serialized_point).unwrap();
    println!("Deserialized point: {:?}", deserialzed_point);

    let message = Message::Request {
        id: "1".to_string(),
        method: "inc".to_string(),
        params: Params {
            names: vec!["count".to_string()],
            values: vec!["1".to_string()],
        },
    };

    let serialized_message = serde_json::to_string(&message).unwrap();
    println!("Serialized message: {}", serialized_message);

    let deserialized_message : Message = serde_json::from_str(&serialized_message).unwrap();
    println!("Deserialized message: {:?}", deserialized_message);

    println!("--- End module: {}", module_path!());
}
