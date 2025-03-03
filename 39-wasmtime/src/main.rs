mod basic_usage;
mod import_host_func;
mod linear_memory;
mod link_modules;

fn main() {
    basic_usage::test();
    println!("");
    import_host_func::test();
    println!("");
    linear_memory::test();
    println!("");
    link_modules::test();
}
