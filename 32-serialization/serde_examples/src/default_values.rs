use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Request {
    // If the value is not present when deserializing, call a function to get a default value.
    #[serde(default = "default_resource")]
    resource: String,

    // If the value is not present when deserializing, use the Default::default().
    #[serde(default)]
    timeout: Timeout,

    // If the value is not present when deserializing, call a function to get a default value.
    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

fn default_resource() -> String {
    "/".to_string()
}

#[derive(Debug, Deserialize)]
struct Timeout(u32);

impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum Priority {
    ExtraHigh,
    High,
    Normal,
    Low,
    ExtraLow,
}

impl Priority {
    fn lowest() -> Self {
        Priority::ExtraLow
    }
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // r# for raw string in Rust.
    // https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let json_string = r#"
        [
            {
                "resource": "/users"
            },
            {
                "timeout": 5,
                "priority": "High"
            }
        ]
    "#;

    let requests: Vec<Request> = serde_json::from_str(json_string).unwrap();

    assert_eq!(requests[0].resource, "/users");
    assert_eq!(requests[0].timeout.0, 30);
    assert_eq!(requests[0].priority, Priority::ExtraLow);

    assert_eq!(requests[1].resource, "/");
    assert_eq!(requests[1].timeout.0, 5);
    assert_eq!(requests[1].priority, Priority::High);

    println!("Request 0 : {:?}", requests[0]);
    println!("Request 1 : {:?}", requests[1]);

    println!("--- End module: {}", module_path!());
}
