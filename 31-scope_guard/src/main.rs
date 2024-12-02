use scopeguard::{self, defer};
use std::{cell::Cell, fs::*, io::{self, Write}};

fn test_guard() {
    // Can be rewritten with `defer!`: defer!{ println!("Exiting guard()."); }
    let _guard = scopeguard::guard((), |_| {
        println!("Exiting guard().");
    });
    
    println!("In guard().");
}

fn test_defer() {
    let drop_counter = Cell::new(0);
    {
        // Use defer! to create a guard.
        defer! {
            drop_counter.set(1 + drop_counter.get());
        }

        // Adding 1 has been deferred to existing the scope.
        assert_eq!(drop_counter.get(), 0);
    }    

    assert_eq!(drop_counter.get(), 1);
}

fn test_guard_with_file() -> io::Result<()> {
    // Create a new file.
    let file = File::create("newfile.txt")?;

    // Create a guard.
    let mut file_guard = scopeguard::guard(file, |file| {
        // Flush the file while exsting the scope.
        let _ = file.sync_all();
    });

    // Access the file through the scope guard.
    file_guard.write_all(b"Test\n")
}

fn main() {
    test_guard();
    test_defer();
    test_guard_with_file().unwrap();
}
