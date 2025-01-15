use std::collections::HashMap;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut scores = HashMap::new();

    let team_blue = String::from("Blue");

    scores.insert(team_blue, 10);
    // Below will cause an error because team_blue has been moved.
    // println!("{}", team_blue);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // This will update the value of the key "Blue" to 20.
    scores.insert(String::from("Blue"), 20);

    // This will insert the key "Blue" with the value 25 only if the key does not exist.
    scores.entry(String::from("Blue")).or_insert(25);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} for {}", score, team_name);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference to the value for this key.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    println!("--- End module: {}", module_path!());
}
