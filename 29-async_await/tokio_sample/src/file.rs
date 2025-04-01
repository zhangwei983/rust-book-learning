use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn test() -> io::Result<()> {
    println!("--- Start module: {}", module_path!());

    // Write to a file asynchronously.
    let mut file = File::create("hello.txt").await?;
    file.write_all(b"Hello, world!").await?;

    // Read from a file asynchronously.
    let mut file = File::open("hello.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("File contents: {}", contents);

    println!("--- End module: {}", module_path!());

    Ok(())
}
