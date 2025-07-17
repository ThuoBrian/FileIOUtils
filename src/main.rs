use std::fs::File;
use std::io::prelude::*;

fn main() {

    //file path:
    let file_path: &str = "src/input.rtf";
    
    let mut f: File = File::open(file_path).expect("Could not read the .txt file");

    // Extract and print the text content
    let mut text_content:String  = String::new();

    // Extract and print the text content
    f.read_to_string(&mut text_content).expect("Could not extract text from the .txt file");
    println!("\nThe file contents as below:: \n\n{}", text_content);
}
