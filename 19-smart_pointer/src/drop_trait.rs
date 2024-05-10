struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`.", self.data);
    }
}

fn test_drop_internal() {
    let csp0 = CustomSmartPointer {
        data: String::from("Create first"),
    };

    let _csp1 = CustomSmartPointer {
        data: String::from("Create second"),
    };

    println!("CustomSmartPointers created.");

    drop(csp0);

    println!("End of test_drop_internal()");
}

pub fn test_drop() {
    println!("--- Start module: {}", module_path!());

    test_drop_internal();

    println!("--- End module: {}", module_path!());
}
