use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum CustomError {
    #[error("Failed to read the key file")]
    FileReadError(#[source] std::io::Error),

    #[error("Failed to send the request")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to delete the key file")]
    FileDeleteError(#[source] std::io::Error),
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?; // This wil call the `Display::fmt` function on CustomError.
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n{}", source)?;
        }
        Ok(())
    }
}

fn make_request() -> Result<(), CustomError> {
    use CustomError::*;

    let key = std::fs::read_to_string("some-key-file").map_err(FileReadError)?;
    reqwest::blocking::get(format!("https://httpbin.org/key/{}", key))?.error_for_status()?;
    std::fs::remove_file(key).map_err(FileDeleteError)?;
    Ok(())
}

pub fn test_with_thiserror() {
    println!("--- Start module: {}", module_path!());

    println!("{:?}", make_request().unwrap_err());

    println!("--- End module: {}", module_path!());
}
