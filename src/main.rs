use std::env;
use std::fs;
use divinity::{evaluate_ast_from_json, generate_ast_from_div};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a file path argument is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <ast|run> <file_path>", args[0]);
        std::process::exit(1);
    }

    // The file path is the second argument
    let file_path = &args[2];

    // Read the file content to a String
    let content = fs::read_to_string(file_path).expect("Failed to read the file");

    let mode = args[1].as_str();
    match mode {
        "ast" => {generate_ast_from_div(content);},
        "run" => {evaluate_ast_from_json(file_path);},
        _ => panic!("invalid command line argument")
    }
}
