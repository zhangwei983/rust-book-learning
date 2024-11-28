mod without_thiserror;
mod with_thiserror;

fn main() {
    without_thiserror::test_without_thiserror();
    println!("");
    with_thiserror::test_with_thiserror();
}
