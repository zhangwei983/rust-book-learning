mod patterns;
mod refutability;
mod pattern_syntax;

fn main() {
    patterns::test_pattern_matching();
    println!("");
    refutability::test_refutability();
    println!("");
    pattern_syntax::test_pattern_syntax();
}
