use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // Several panics.
    // panic!("Crash and burn");

    let _v = vec![1, 2, 3];
    // _v[10]; // panic! from a library as accessing an index beyond the range.

    let hello_file_name: String = String::from("hello.txt");

    // Try to recover with Result.
    let open_file_result = File::open(&hello_file_name);
    let _hello_file = match open_file_result {
        Ok(file) => {
            println!("File opened");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&hello_file_name) { // Try to creat the file instead of panic! directly.
                Ok(file) => {
                    println!("File created");
                    file
                }
                Err(e) => panic!("Creation failed with error: {}", e),
            }
            other => panic!("Open failed with {}", other),
        }
    };

    println!("{:?}", _hello_file);

    // Usage of unwrap() to simplify the matching logic.
    // let _greeting_file = File::open("greeting_file.txt").unwrap();

    // Use expect() to set the error message.
    let _greeting_file = File::open("greeting_file.txt").expect("File opened failed.");

}

fn _read_username_from_file(file_name: &String) -> Result<String, io::Error> {

    let mut username_file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// Simplied version with using `?` operator.
fn _read_username_from_file_simplied(file_name: &String) -> Result<String, io::Error> {

    let mut username = String::new();
    File::open(file_name)?.read_to_string(&mut username)?; 

    Ok(username)
}

// Simplest version with using `?` operator.
fn _read_username_from_file_simplest(file_name: &String) -> Result<String, io::Error> {

    fs::read_to_string(file_name)
}
