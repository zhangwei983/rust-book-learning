mod basic_usage;
mod import_host_func;
mod linear_memory;
mod link_modules;
mod wasm_component;

fn main() {
    basic_usage::test();
    println!("");
    import_host_func::test();
    println!("");
    linear_memory::test();
    println!("");
    link_modules::test();
    println!("");
    wasm_component::test();
    println!("");
    wasm_component::test1();
}
