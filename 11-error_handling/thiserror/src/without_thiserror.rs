use std::{error::Error, fmt::Debug};

pub enum CustomError {
    FileReadError(std::io::Error),
    RequestError(reqwest::Error),
    FileDeleteError(std::io::Error),
}

impl From<reqwest::Error> for CustomError {
    fn from(e: reqwest::Error) -> Self {
        CustomError::RequestError(e)
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FileReadError(_) => write!(f, "Failed to read the key file"),
            CustomError::RequestError(_) => write!(f, "Failed to send the request"),
            CustomError::FileDeleteError(_) => write!(f, "Failed to delete the key file"),
        }
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CustomError::FileReadError(s) => Some(s),
            CustomError::RequestError(s) => Some(s),
            CustomError::FileDeleteError(s) => Some(s),
        }
    }
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?; // This wil call the `fmt` function on CustomError.
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

pub fn test_without_thiserror() {
    println!("--- Start module: {}", module_path!());

    println!("{:?}", make_request().unwrap_err());

    println!("--- End module: {}", module_path!());
}
