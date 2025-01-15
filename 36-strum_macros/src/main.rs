mod enum_as_ref;
mod enum_discriminants;
mod enum_display;
mod enum_is;
mod enum_iter;
mod enum_string;

fn main() {
    enum_string::test();
    println!("");
    enum_iter::test();
    println!("");
    enum_display::test();
    println!("");
    enum_as_ref::test();
    println!("");
    enum_is::test();
    println!("");
    enum_discriminants::test();
}
