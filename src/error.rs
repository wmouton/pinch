use std::fmt;
use std::io;
use std::path::PathBuf;

/// A custom result type for this application.
pub type Result<T> = std::result::Result<T, PinchError>;

/// The custom error type for the pinch application.
/// It enumerates all possible errors that can occur.
#[derive(Debug)]
pub enum PinchError {
    /// Error when a given path is not a directory when it should be.
    NotADirectory(PathBuf),
    /// Error when a given path is not a file when it should be.
    NotAFile(PathBuf),
    /// Error when a file is not a valid .tar.gz archive.
    NotAnArchive(PathBuf),
    /// Error when a directory or file name cannot be determined.
    InvalidFileName(PathBuf),
    /// An error occurred during an I/O operation (e.g., creating a directory).
    Io(io::Error),
    /// The external 'tar' command failed to execute.
    TarExecutionError {
        stdout: String,
        stderr: String,
    },
}

// Allow easy conversion from std::io::Error to our custom PinchError.
// This allows us to use the `?` operator on I/O operations.
impl From<io::Error> for PinchError {
    fn from(err: io::Error) -> Self {
        PinchError::Io(err)
    }
}

// Implement the Display trait to provide user-friendly error messages.
impl fmt::Display for PinchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PinchError::NotADirectory(path) => write!(f, "'{}' is not a directory.", path.display()),
            PinchError::NotAFile(path) => write!(f, "'{}' is not a file.", path.display()),
            PinchError::NotAnArchive(path) => write!(f, "'{}' does not appear to be a .tar.gz file.", path.display()),
            PinchError::InvalidFileName(path) => write!(f, "Could not determine a valid name from the path '{}'.", path.display()),
            PinchError::Io(err) => write!(f, "I/O Error: {}", err),
            PinchError::TarExecutionError { stdout, stderr } => {
                write!(f, "Failed to execute 'tar' command.\n--- STDOUT ---\n{}\n--- STDERR ---\n{}", stdout, stderr)
            }
        }
    }
}

// Implement the Error trait to be compatible with other error-handling logic.
impl std::error::Error for PinchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PinchError::Io(err) => Some(err),
            _ => None,
        }
    }
}
