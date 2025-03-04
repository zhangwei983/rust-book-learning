use backoff::{Error, ExponentialBackoff};
use reqwest::Url;
use std::fmt::Display;
use std::io::{self, Read};

fn new_io_err<E: Display>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.to_string())
}

fn fetch_url(url: &str) -> Result<String, Error<io::Error>> {
    let op = || {
        println!("Fetching {}", url);
        let url = Url::parse(url)
            .map_err(new_io_err)
            .map_err(Error::Permanent)?;

        let mut response = reqwest::blocking::get(url).map_err(new_io_err)?;

        let mut content = String::new();
        let _ = response.read_to_string(&mut content);
        Ok(content)
    };

    let retry = ExponentialBackoff::default();
    backoff::retry(retry, op)
}

fn main() {
    match fetch_url("https::///wrong URL") {
        Ok(_) => println!("Successfully fetched"),
        Err(err) => println!("Failed to fetch: {}", err),
    }

    match fetch_url("https://www.google.com") {
        Ok(_) => println!("Successfully fetched"),
        Err(err) => println!("Failed to fetch: {}", err),
    }
}
