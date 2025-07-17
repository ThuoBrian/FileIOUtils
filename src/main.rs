use std::fs::File;
use std::io::prelude::*;

const FILE_EXTENSION: &str = "/Users/brianthuo/Documents/GitHub/FileIOUtils/input_text.rtf";

fn main() {
    //file path:
    let file_path: &str = FILE_EXTENSION;

    let mut f = match File::open(file_path) {
        Ok(file) => {
            println!("File opened successfully.");
            file
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("File not found: {}", file_path);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied: {}", file_path);
                }
                _ => eprintln!("An error occurred while opening the file: {}", e),
            }
            return;
        }
    };

    // Extract and print the text content
    let mut text_content: String = String::new();

    f.read_to_string(&mut text_content)
        .expect("Could not extract text from the .rft file");
    println!("\nThe file contents as below:: \n\n{}", text_content);
}
