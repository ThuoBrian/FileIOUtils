use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

/// Reads the entire contents of a file at the given path as a UTF-8 string.
///
/// # Arguments
///
/// * `file_path` - A string slice holding the path to the file.
///
/// # Returns
///
/// A `Result<String, io::Error>` containing the file contents or an error.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or if its contents are not valid UTF-8.
fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    // Get file path from command-line arguments
    let file_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("❗ Usage: cargo run <file_path>");
        process::exit(1);
    });

    match read_file_to_string(&file_path) {
        Ok(contents) => {
            println!(
                "✅ File opened successfully.\n\n📄 File Contents:\n\n{}",
                contents
            );
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("❌ File not found: '{}'", file_path);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("🔒 Permission denied when accessing: '{}'", file_path);
                }
                io::ErrorKind::InvalidData => {
                    eprintln!("⚠️ File contains non-UTF8 data: '{}'", file_path);
                }
                _ => {
                    eprintln!("⚠️ Failed to read '{}': {}", file_path, e);
                }
            }
            process::exit(1);
        }
    }
}
