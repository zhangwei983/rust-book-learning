use std::fs; 
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", _http_request);

    // let response = "HTTP/1.1 200 OK\r\n\r\n";

    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let(status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let len = content.len();

    let response = 
        format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
