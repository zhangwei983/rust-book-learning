mod cfg_attribute;
mod cfg_macro;

fn main() {
    cfg_attribute::test_cfg_attribute();
    println!("");
    cfg_macro::test_cfg_macro();    
}
