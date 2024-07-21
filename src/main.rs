// src/main.rs
mod errors;
mod file_utils;
mod errors;

use errors::PolyYieldSecureError;
use crate::file_utils::read_file;

// fn main() {
//     match read_file("some_file.txt") {
//         Ok(contents) => println!("File contents: {}", contents),
//         Err(e) => eprintln!("Error reading file: {}", e),
//     }
// }


fn main() {
    match errors::example_function() {
        Ok(_) => println!("Function executed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
