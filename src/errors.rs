// src/errors.rs
use std::fmt;

#[derive(Debug)]
pub enum PolyYieldSecureError {
    IoError(std::io::Error),
    // Add more error types as needed
}

impl fmt::Display for PolyYieldSecureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolyYieldSecureError::IoError(err) => write!(f, "IO Error: {}", err),
            // Add more error descriptions as needed
        }
    }
}

impl From<std::io::Error> for PolyYieldSecureError {
    fn from(err: std::io::Error) -> PolyYieldSecureError {
        PolyYieldSecureError::IoError(err)
    }
}
