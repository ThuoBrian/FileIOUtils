//! # FileIOUtils
//!
//! `file_io_utils` is a small utility crate for reading text from files.
//!
//! This crate provides a function to load file contents into a `String`.

use std::fs::File;
use std::io::{self, Read};

/// Opens a file at the given path and returns its contents as a String.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to open.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
///
/// # Example
///
/// ```no_run
/// let contents = file_io_utils::read_file_to_string("/path/to/file.txt").unwrap();
/// println!("{}", contents);
/// ```
pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
