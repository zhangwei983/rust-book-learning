pub fn test() {
    println!("--- Start module: {}", module_path!());

    let s1 = "Hello".to_string();
    let s2: String = s1.to_owned();
    assert_eq!(s1, s2);
    println!("{}", s1); // s1 is not moved to to_owned.

    let v1 = vec![1, 2, 3];
    let v2 = v1.to_owned();
    assert_eq!(v1, v2);
    println!("{:?}", v1); // v1 is not moved to to_owned.

    println!("--- End module: {}", module_path!());
}
