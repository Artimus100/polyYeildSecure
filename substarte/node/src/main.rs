//! Main entry point for the node.

// Import required dependencies
use sc_cli::SubstrateCli;
use node_template_runtime::Block;
use node_template_service::new_partial;
use sc_service::{Configuration, error::Error as ServiceError};

// Main function
fn main() -> sc_cli::Result<()> {
    // Define the CLI structure
    let cli = cli::Cli::from_args();
    let runner = cli.create_runner(&cli.run)?;

    // Run the node
    runner.run_node_until_exit(|config| async move {
        let partial = new_partial(&config)?;
        Ok(partial)
    })
}

// Define CLI structure and options
mod cli {
    use structopt::StructOpt;
    use sc_cli::RunCmd;

    #[derive(Debug, StructOpt)]
    pub struct Cli {
        #[structopt(flatten)]
        pub run: RunCmd,
    }

    impl SubstrateCli for Cli {
        fn impl_name() -> &'static str {
            "Substrate Node"
        }

        fn impl_version() -> &'static str {
            env!("CARGO_PKG_VERSION")
        }
    }
}
