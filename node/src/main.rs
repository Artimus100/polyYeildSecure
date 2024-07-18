//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod benchmarking;
mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;
use polyyieldsecure_errors::{PolyYieldSecureError, example_function};

fn main() -> sc_cli::Result<()> {

	Ok(if let Err(err) = example_function() {
        eprintln!("Error occurred: {}", err);
        if let Some(source) = err.source() {
            eprintln!("Caused by: {}", source);
        }
        std::process::exit(1);
    })

}
