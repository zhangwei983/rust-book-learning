mod r#box;
mod deref_trait;
mod drop_trait;
mod rc;
mod refcell;
mod rc_refcell;
mod ref_cycle;

fn main() {
    r#box::test_box_t();
    println!("");
    deref_trait::test_deref();
    println!("");
    drop_trait::test_drop();
    println!("");
    rc::test_rc_t();
    println!("");
    rc_refcell::test_rc_refcell();
    println!("");
    ref_cycle::test_ref_cycle();
}
