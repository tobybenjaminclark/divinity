use std::fs::{self, File};
use std::io::Write;
use lalrpop_util::lalrpop_mod;
use crate::compile::ast::Program;
use crate::compile::interpreter::{evaluate_program, Var};

// Import the ast module
mod compile;

lalrpop_mod!(pub calculator1);
pub fn generate_ast_from_div(content: String) -> i32 {
    // Try to parse the .div file content into an AST
    let ast = match calculator1::ProgramParser::new().parse(content.as_str()) {
        Ok(ast) => ast,
        Err(_) => {
            // If parsing fails, return 0
            eprintln!("Failed to parse the .div file");
            return 0;
        }
    };

    // Serialize the AST to a JSON string
    let json_str = match serde_json::to_string(&*ast) {
        Ok(json) => json,
        Err(_) => {
            // If serialization fails, return 0
            eprintln!("Failed to serialize the AST to JSON");
            return 0;
        }
    };

    // Write the JSON string to a file
    let mut file = match File::create("ast_output.json") {
        Ok(f) => f,
        Err(_) => {
            // If file creation fails, return 0
            eprintln!("Failed to create 'ast_output.json' file");
            return 0;
        }
    };

    if let Err(_) = file.write_all(json_str.as_bytes()) {
        // If writing to file fails, return 0
        eprintln!("Failed to write the AST to the file");
        return 0;
    }

    // If everything is successful, print success message
    println!("Serialized AST written to file 'ast_output.json'");

    // Return 1 to indicate success
    1
}

/// This function takes in the content of a JSON file, parses it into an AST,
/// and then evaluates it.
pub fn evaluate_ast_from_json(file_path: &str) -> Var {
    // Read the JSON AST from the given file
    let content = fs::read_to_string(file_path)
        .expect("Failed to read the JSON AST file");

    // Parse the JSON content into an AST
    let ast: Program = serde_json::from_str(&content)
        .expect("Failed to parse the JSON AST");

    // Evaluate the parsed AST
    let ret = evaluate_program(ast, false);

    println!("Result: {}", ret);

    // Return the evaluation result
    return ret;
}
