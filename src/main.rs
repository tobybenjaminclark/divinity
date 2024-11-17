use std::env;
use std::fs;
use divinity::calculator4;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path argument is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // The file path is the second argument
    let file_path = &args[1];

    // Read the file content to a String
    let content = fs::read_to_string(file_path).expect("Failed to read the file");

    // Process the file content
    let output = calculator4(content);
    println!("Result: {}", output);
}
