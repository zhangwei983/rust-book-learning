pub fn test_cfg_macro() {
    println!("--- Start module: {}", module_path!());

    // println!("{}", std::env::consts::OS);

    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "MacOS"
    } else {
        "unknown"
    };

    println!("Current OS: {}", os);

    println!("--- Start module: {}", module_path!());
}
