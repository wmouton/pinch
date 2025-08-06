use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A simple command-line tool to compress and extract directories using tar.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Defines the subcommands for 'pinch'
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Extracts a .tar.gz archive into a directory with the same name.
    Extract {
        /// The path to the .tar.gz archive to extract.
        archive_path: PathBuf,
    },
    /// Compresses a directory into a .tar.gz archive with the same name as the directory.
    Compress {
        /// The path to the directory to compress.
        directory_path: PathBuf,
    },
}
