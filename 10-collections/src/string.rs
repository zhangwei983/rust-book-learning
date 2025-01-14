pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut s = String::new();
    s.push_str("Hello");
    s.push(' ');
    s.push_str("world!");

    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    // fn add(self, s: &str) -> String {
    // The compiler can coerce the &String argument into a &str.
    let s3 = s1 + &s2;

    // s1 is moved, so it cannot be used here
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! won't take ownership of the arguments, returns a String.
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = String::from("Hola");
    println!("{}", hello.len());

    let hello = "Здравствуйте";
    println!("{}", hello.len());
    // Rust does not support indexing into a string.
    // let answer = &hello[0];
    let s = &hello[0..4];
    println!("{}", s);

    // The below will panic because byte index 1 is not a char boundary.
    // let s = &hello[0..1];
    // println!("{}", s);

    // It's important to remember that [char] represents a Unicode Scalar Value, and might not match your idea of what a 'character' is.
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    println!("--- End module: {}", module_path!());
}
