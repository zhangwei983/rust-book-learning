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

fn main() {
    let _r;
    _r = 5;

    // Usage of lifetime annotation.
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
