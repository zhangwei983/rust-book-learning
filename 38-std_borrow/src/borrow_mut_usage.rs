use std::borrow::BorrowMut;

fn check<T: BorrowMut<[i32]>>(mut v: T) {
    let v = v.borrow_mut();
    v[0] = 2;
    assert_eq!(&mut [2, 2, 3], v);
    println!("{:?}", v);
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let v = vec![1, 2, 3];
    check(v);
    // println!("{:?}", v); // Won't compile as v is moved to check.

    println!("--- End module: {}", module_path!());
}
