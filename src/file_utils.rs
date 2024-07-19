// src/file_utils.rs
use std::fs::File;
use std::io::{self, Read};
use crate::errors::PolyYieldSecureError;

pub fn read_file(path: &str) -> Result<String, PolyYieldSecureError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
