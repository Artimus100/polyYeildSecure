use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolyYieldSecureError {
    #[error("An error occurred: {0}")]
    GeneralError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    // Add more error variants as needed
}

// Example function that may encounter errors
pub fn example_function() -> Result<(), PolyYieldSecureError> {
    // Simulating an IO error
    let _result = std::fs::read_to_string("nonexistent_file.txt")?;
    Ok(())
}
// src/errors.rs
// use std::fmt;

// #[derive(Debug)]
// pub enum PolyYieldSecureError {
//     IoError(std::io::Error),
//     // Add more error types as needed
// }

// impl fmt::Display for PolyYieldSecureError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PolyYieldSecureError::IoError(err) => write!(f, "IO Error: {}", err),
//             // Add more error descriptions as needed
//         }
//     }
// }

// impl From<std::io::Error> for PolyYieldSecureError {
//     fn from(err: std::io::Error) -> PolyYieldSecureError {
//         PolyYieldSecureError::IoError(err)
//     }
// }
