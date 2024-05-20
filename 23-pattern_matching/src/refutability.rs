pub fn test_refutability() {
    println!("--- Start module: {}", module_path!());

    // This won't compile as let requires an "irrefutable pattern".
    // let Some(s) = return_option(true); 
    
    // Use `if let` to match a refutable pattern.
    if let Some(s) = return_option(true) {
        println!("{}", s);
    }

    // This gets a warning for using `if let` to match an "irrefutable pattern". 
    // if let _x = 5 {}

    println!("--- End module: {}", module_path!());
}

fn return_option(valid: bool) -> Option<String>{
    if valid {
        Some(String::from("Valid"))
    } else {
        None
    }
}
