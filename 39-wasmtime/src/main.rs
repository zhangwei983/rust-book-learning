mod basic_usage;
mod import_host_func;
mod linear_memory;

fn main() {
    basic_usage::test();
    println!("");
    import_host_func::test();
    println!("");
    linear_memory::test();
}
