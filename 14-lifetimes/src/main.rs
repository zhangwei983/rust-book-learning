/* The below code can't compile as Rust can’t tell whether the reference being returned refers to x or y.
fn longest(x: &str, y: &str) -> &str { 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/

// Lifetime annotations in function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let _r;
    _r = 5;

    // Usage of lifetime annotation in function.
    let string1 = String::from("abcd");

    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // println!("The longest string is {}", result); // This won't compile.

    // Usage of lifetime annotation in struct.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("First sentence is :{}", i.part);
}
