use wasmtime::*;

struct Log {
    integers_logged: Vec<i32>,
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let engine = Engine::default();
    let module = Module::from_file(&engine, "wasms/run.wat").unwrap();

    // Create a Linker to register host functions.
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("", "double", |param: i32| param * 2)
        .unwrap();
    linker
        .func_wrap("", "log", |mut caller: Caller<'_, Log>, param: i32| {
            println!("log: {}", param);
            caller.data_mut().integers_logged.push(param);
        })
        .unwrap();

    // Initialize the store with the custom data.
    let data = Log {
        integers_logged: Vec::new(),
    };
    let mut store = Store::new(&engine, data);

    // Instantiate the module with the host functions.
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let run = instance
        .get_typed_func::<(), ()>(&mut store, "run")
        .unwrap();

    // Call the function.
    run.call(&mut store, ()).unwrap();
    println!("Logged integers: {:?}", store.data().integers_logged);

    println!("--- End module: {}", module_path!());
}
