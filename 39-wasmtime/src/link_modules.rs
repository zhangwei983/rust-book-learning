use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let engine = Engine::default();

    // Create a linker that can link multiple modules together.
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s).unwrap();

    let linking1 = Module::from_file(&engine, "wasms/linking1.wat").unwrap();
    let linking2 = Module::from_file(&engine, "wasms/linking2.wat").unwrap();

    // Configure WASI and add to the store.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate the first module.
    let linking2 = linker.instantiate(&mut store, &linking2).unwrap();

    // Register the instance with the linker for next linking.
    linker.instance(&mut store, "linking2", linking2).unwrap();

    // Perform the final linking.
    let linking1 = linker.instantiate(&mut store, &linking1).unwrap();

    // Call the `run` function exported by the first module.
    let run = linking1
        .get_typed_func::<(), ()>(&mut store, "run")
        .unwrap();
    run.call(&mut store, ()).unwrap();

    println!("--- End module: {}", module_path!());
}
