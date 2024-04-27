
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3];

    let iter_v1 = v1.iter();

    // Use iterator.
    for value in iter_v1 {
        println!("Value: {value}");
    }

    // Use `next` method.
    // mut is necessary as calling the next method on an iterator changes internal state
    // that the iterator uses to keep track of where it is in the sequence. 
    let mut mut_iter_v1 = v1.iter();

    assert_eq!(mut_iter_v1.next(), Some(&1));
    assert_eq!(mut_iter_v1.next(), Some(&2));
    assert_eq!(mut_iter_v1.next(), Some(&3));
    assert_eq!(mut_iter_v1.next(), None);

    // Example of `consuming adaptors`, `sum()` takes the ownership of `iter_v1`.
    let iter_v1 = v1.iter();
    let total: i32 = iter_v1.sum();

    assert_eq!(total, 6);
    println!("Sum is: {total}");
    
    // println!("{:?}", iter_v1); // This won't compile as iter_v1.sum() takes the ownership of `iter_v1`.

    // Example of `iterator adaptors`, `map` takes the ownership of `iter_v1` and returns a new iterator.
    let iter_v1 = v1.iter();

    let new_iter = iter_v1.map(|x| x + 1);
    println!("{:?}", new_iter);

    // Call the `lazy` iterator to take effect.
    let v2: Vec<_> = new_iter.collect();
    assert_eq!(v2, vec![2, 3, 4]);
    println!("{:?}", v2);

}
