use crate::error::{PinchError, Result};
use std::path::Path;
use std::process::Command;

/// Compresses a given directory into a .tar.gz archive.
pub fn compress_directory(directory_path: &Path) -> Result<()> {
    if !directory_path.is_dir() {
        return Err(PinchError::NotADirectory(directory_path.to_path_buf()));
    }

    let dir_name = directory_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| PinchError::InvalidFileName(directory_path.to_path_buf()))?;

    let archive_name = format!("{}.tar.gz", dir_name);

    let parent_dir = directory_path.parent().unwrap_or_else(|| Path::new("."));

    println!("Compressing '{}' into '{}'...", directory_path.display(), archive_name);

    let output = Command::new("tar")
        .arg("-czvf")
        .arg(&archive_name)
        .arg("-C")
        .arg(parent_dir)
        .arg(dir_name)
        .output()?; // The '?' will convert an IO error to PinchError::Io

    if !output.status.success() {
        return Err(PinchError::TarExecutionError {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(())
}

/// Extracts a given .tar.gz archive into a directory.
pub fn extract_archive(archive_path: &Path) -> Result<()> {
    if !archive_path.is_file() {
        return Err(PinchError::NotAFile(archive_path.to_path_buf()));
    }

    let file_name = archive_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| PinchError::InvalidFileName(archive_path.to_path_buf()))?;

    if !file_name.ends_with(".tar.gz") {
        return Err(PinchError::NotAnArchive(archive_path.to_path_buf()));
    }

    let output_dir_name = file_name.strip_suffix(".tar.gz").unwrap(); // Safe due to check above
    let output_dir_path = archive_path.parent().unwrap_or_else(|| Path::new(".")).join(output_dir_name);

    if !output_dir_path.exists() {
        println!("Creating output directory: '{}'", output_dir_path.display());
        std::fs::create_dir_all(&output_dir_path)?;
    }

    println!("Extracting '{}' into '{}'...", archive_path.display(), output_dir_path.display());

    let output = Command::new("tar")
        .arg("-xzvf")
        .arg(archive_path)
        .arg("-C")
        .arg(&output_dir_path)
        .output()?; // The '?' will convert an IO error to PinchError::Io

    if !output.status.success() {
        return Err(PinchError::TarExecutionError {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(())
}
