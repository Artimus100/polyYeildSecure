use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolyYieldSecureError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Nix Error: {0}")]
    NixError(#[from] nix::Error),

    #[error("Custom Error: {0}")]
    Custom(String),
}

impl fmt::Display for PolyYieldSecureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolyYieldSecureError::IoError(e) => write!(f, "IO Error: {}", e),
            PolyYieldSecureError::NixError(e) => write!(f, "Nix Error: {}", e),
            PolyYieldSecureError::Custom(e) => write!(f, "Custom Error: {}", e),
        }
    }
}
