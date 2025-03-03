use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::bindings::sync::Command;
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl IoView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub fn test() {
    println!("--- Start module 0: {}", module_path!());

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

    // Create a WASI context and store in the Store.
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    // Instantiate the WASM component as a Command to run a wasi:cli command.
    let component = Component::from_file(&engine, "wasms/wasm_component.wasm").unwrap();
    let command = Command::instantiate(&mut store, &component, &linker).unwrap();

    // Run the program.
    let program_result = command.wasi_cli_run().call_run(&mut store).unwrap();
    if program_result.is_err() {
        std::process::exit(1);
    }

    println!("--- End module 0: {}", module_path!());
}

pub fn test1() {
    println!("--- Start module 1: {}", module_path!());

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();

    // Create a WASI context and store in the Store.
    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    // Instantiate the component as a normal component.
    let component = Component::from_file(&engine, "wasms/wasm_component.wasm").unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    // Get index for the exported interface.
    let interface_idx = instance
        .get_export(&mut store, None, "wasi:cli/run@0.2.0")
        .unwrap();

    // Get index for the exported function in the exported interface.
    let parent_export_idx = Some(&interface_idx);
    let func_idx = instance
        .get_export(&mut store, parent_export_idx, "run")
        .unwrap();

    // Get the function.
    // The `run` function in `wasi:cli/run@0.2.0` takes no argument and return a WASI `Result<(), ()>` result.
    let func = instance
        .get_typed_func::<(), (Result<(), ()>,)>(&mut store, func_idx)
        .unwrap();

    // Call the function.
    let (_result,) = func.call(&mut store, ()).unwrap();
    func.post_return(&mut store).unwrap();

    println!("--- End module 1: {}", module_path!());
}
