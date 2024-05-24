pub fn test_advanced_types() {
    println!("--- Start module: {}", module_path!());

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    println!("--- End module: {}", module_path!());
}
