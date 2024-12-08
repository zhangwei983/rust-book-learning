use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,

    // Flatten the HashMap.
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Users {
    // Flatten pagination.
    #[serde(flatten)]
    pagination: Pagination,

    users: Vec<User>,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let users = Users {
        pagination: Pagination {
            limit: 100,
            offset: 200,
            total: 1053,
        },
        users: vec![User {
            id: "0".to_string(),
            username: "Vincent".to_string(),
            extra: HashMap::from([("location".to_string(), json!("Shanghai"))]),
        }],
    };

    let users_json = serde_json::to_string(&users).unwrap();
    println!("{}", users_json);

    println!("--- End module: {}", module_path!());
}
