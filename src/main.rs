use std::fs;
use divinity::calculator4;

fn main() {
    // Specify the file path
    let file_path = "program.div";

    // Read the file content to a String
    let content = fs::read_to_string(file_path).unwrap();

    let output = calculator4(content);
    println!("Result: {}", output);
}
