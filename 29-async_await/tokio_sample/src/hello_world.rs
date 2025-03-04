async fn say_world() {
    println!("world");
}

pub async fn test() {
    println!("--- Start module: {}", module_path!());

    // Calling an async function but not executing it.
    let world = say_world();

    // This comes first.
    print!("hello ");

    // Calling await to execute the async function.
    world.await;

    println!("--- End module: {}", module_path!());
}
