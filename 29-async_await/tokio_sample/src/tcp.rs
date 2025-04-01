use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const LISTENING_ADDRESS: &str = "127.0.0.1:8081";

async fn start_server() -> Result<(), tokio::io::Error> {
    let listener = TcpListener::bind(LISTENING_ADDRESS).await?;
    let (mut socket, _) = listener.accept().await?;

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("Server received: {}", String::from_utf8_lossy(&buf[..n]));

    socket.write_all(b"Hello from server!").await?;

    Ok(())
}

async fn start_client() -> Result<(), tokio::io::Error> {
    let mut stream = TcpStream::connect(LISTENING_ADDRESS).await?;
    stream.write_all(b"Hello from client!").await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Client received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Start module: {}", module_path!());

    let server = tokio::spawn(start_server());
    let client = tokio::spawn(start_client());

    let _ = tokio::try_join!(server, client)?;

    println!("--- End module: {}", module_path!());

    Ok(())
}
