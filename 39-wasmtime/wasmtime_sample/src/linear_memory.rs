use wasmtime::*;

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let mut store: Store<()> = Store::default();
    let module = Module::from_file(store.engine(), "wasms/memory.wat").unwrap();
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let size_fn = instance
        .get_typed_func::<(), i32>(&mut store, "size")
        .unwrap();
    let load_fn = instance
        .get_typed_func::<i32, i32>(&mut store, "load")
        .unwrap();
    let store_fn = instance
        .get_typed_func::<(i32, i32), ()>(&mut store, "store")
        .unwrap();

    println!("Checking memory...");
    assert_eq!(memory.size(&store), 2); // Page size(64KB - 0x10000).
    assert_eq!(memory.data_size(&store), 0x20000); // Memory size in bytes.
    assert_eq!(memory.data_mut(&mut store)[0], 0);
    assert_eq!(memory.data_mut(&mut store)[0x1000], 1);
    assert_eq!(memory.data_mut(&mut store)[0x1003], 4);

    assert_eq!(size_fn.call(&mut store, ()).unwrap(), 2);
    assert_eq!(load_fn.call(&mut store, 0).unwrap(), 0);
    assert_eq!(load_fn.call(&mut store, 0x1000).unwrap(), 1);
    assert_eq!(load_fn.call(&mut store, 0x1003).unwrap(), 4);
    assert_eq!(load_fn.call(&mut store, 0x1ffff).unwrap(), 0);
    assert!(load_fn.call(&mut store, 0x20000).is_err()); // Out of bounds.

    println!("Mutating memory...");
    memory.data_mut(&mut store)[0x1003] = 5;
    store_fn.call(&mut store, (0x1002, 6)).unwrap();

    assert_eq!(memory.data(&store)[0x1002], 6);
    assert_eq!(memory.data(&store)[0x1003], 5);
    assert_eq!(load_fn.call(&mut store, 0x1002).unwrap(), 6);
    assert_eq!(load_fn.call(&mut store, 0x1003).unwrap(), 5);

    println!("Growing memory...");
    memory.grow(&mut store, 1).unwrap(); // In page size.
    assert_eq!(memory.size(&store), 3);
    assert_eq!(memory.data_size(&store), 0x30000);
    assert_eq!(load_fn.call(&mut store, 0x20000).unwrap(), 0);

    store_fn.call(&mut store, (0x20000, 0)).unwrap();
    assert!(load_fn.call(&mut store, 0x30000).is_err());
    assert!(store_fn.call(&mut store, (0x30000, 0)).is_err());

    assert!(memory.grow(&mut store, 1).is_err()); // Max size 3 reached.
    assert!(memory.grow(&mut store, 0).is_ok());
    assert_eq!(memory.size(&store), 3);

    println!("Creating stand-alone memory...");
    let memory_type = MemoryType::new(5, Some(5));
    let memory2 = Memory::new(&mut store, memory_type).unwrap();
    assert_eq!(memory2.size(&store), 5);
    assert!(memory2.grow(&mut store, 1).is_err()); // Max reached.
    assert!(memory2.grow(&mut store, 0).is_ok());

    println!("--- End module: {}", module_path!());
}
