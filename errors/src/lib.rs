use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PolyYieldSecureError {
    IoError(std::io::Error),
    // Add more error variants as needed
}

impl Error for PolyYieldSecureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PolyYieldSecureError::IoError(ref err) => Some(err),
            // Add other error sources here
        }
    }
}

impl fmt::Display for PolyYieldSecureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PolyYieldSecureError::IoError(ref err) => write!(f, "IO error: {}", err),
            // Add more formatted error messages as needed
        }
    }
}

// Example function that may encounter errors
pub fn example_function() -> Result<(), PolyYieldSecureError> {
    // Simulating an IO error
    let _result = std::fs::read_to_string("nonexistent_file.txt")
        .map_err(|err| PolyYieldSecureError::IoError(err))?;

    Ok(())
}
use thiserror::Error;
use std::fmt;

pub enum PolyYieldSecureError {
    #[error("An error occurred: {0}")]
    GeneralError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    // Add more error variants as needed
}

