use std::fs::File;
use std::io::{self, Read};
use std::process;

const FILE_PATH: &str = "/Users/brianthuo/Documents/GitHub/FileIOUtils/input_text.rtf";

/// Reads the entire contents of a file at the given path.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// A `Result<String, io::Error>` containing the file contents or an error.
///
/// # Errors
///
/// Returns an error if the file does not exist, cannot be opened, or is unreadable.
fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_to_string(FILE_PATH) {
        Ok(contents) => {
            println!("✅ File opened successfully.\n\n📄 File Contents:\n\n{}", contents);
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("❌ File not found: {}", FILE_PATH);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("🔒 Permission denied: {}", FILE_PATH);
                }
                _ => {
                    eprintln!("⚠️ Failed to read the file: {} — {}", FILE_PATH, e);
                }
            }
            process::exit(1);
        }
    }
}
