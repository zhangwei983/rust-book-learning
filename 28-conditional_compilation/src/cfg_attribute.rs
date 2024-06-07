#[cfg(target_os = "linux")]
fn on_linux() -> bool {
    true
}

#[cfg(not(target_os = "linux"))]
fn on_linux() -> bool {
    false
}

// The test_condition is defined in the ``.cargo/config.toml` file.
#[cfg(test_condition)]
fn conditional_function() {
    println!("Test condition met!");
}

#[cfg(test_condition_1)]
fn conditional_function_1() {
    println!("Test condition 1 met!");
}

pub fn test_cfg_attribute() {
    println!("--- Start module: {}", module_path!());

    let is_on_linux = on_linux();
    println!("Are you on Linux? : {}", is_on_linux);

    conditional_function();
    conditional_function_1();

    println!("--- End module: {}", module_path!());
}
