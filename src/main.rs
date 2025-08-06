// Declare the modules we've created. Rust will look for `cli.rs`, `handler.rs`, and `error.rs`.
mod cli;
mod error;
mod handler;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result; // Use our custom Result type

fn main() {
    // A common pattern: `main` is responsible for running the app
    // and handling any errors that bubble up, then exiting.
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// The primary logic function of the application.
fn run() -> Result<()> {
    // Parse the command-line arguments using the definition from the `cli` module.
    let cli = Cli::parse();

    // Match the subcommand and call the appropriate function from the `handler` module.
    match &cli.command {
        Commands::Compress { directory_path } => {
            handler::compress_directory(directory_path)?;
            println!("Successfully compressed '{}'.", directory_path.display());
        }
        Commands::Extract { archive_path } => {
            handler::extract_archive(archive_path)?;
            println!("Successfully extracted '{}'.", archive_path.display());
        }
    }

    Ok(())
}
