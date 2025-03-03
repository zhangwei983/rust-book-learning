use wasmtime::*;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    // Stores and configures global compilation settings.
    let engine = Engine::default();

    // A `Module` which represents a compiled form of our input wasm module.
    let module = Module::from_file(&engine, "wasms/answer.wat").unwrap();

    // All wasm items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world.
    let mut store = Store::new(&engine, ());

    // Instantiate a compiled module.
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    // Access the exported function.
    let answer_func = instance
        .get_typed_func::<(), i32>(&mut store, "answer")
        .unwrap();

    // Same as get_typed_func().
    // let answer_func = instance.get_func(&mut store, "answer").unwrap();
    // let answer_func = answer_func.typed::<(), i32>(&store).unwrap();

    // Call the function.
    let answer = answer_func.call(&mut store, ()).unwrap();
    println!("Answer: {}", answer);

    println!("--- End module: {}", module_path!());
}
