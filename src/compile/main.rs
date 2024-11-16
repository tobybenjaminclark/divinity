use std::any::Any;
use std::fs;
use crate::ast::*;
use lalrpop_util::lalrpop_mod;
use interpreter::*;
use std::fs::File;
use std::io::Write;

// Import the ast module
mod ast;
mod interpreter;

lalrpop_mod!(pub calculator1);

fn main() {
    calculator4();
    return;
}

fn calculator4() {

    // Specify the file path
    let file_path = "program.div";

    // Read the file content to a String
    let content = fs::read_to_string(file_path).unwrap();

    let ast: Box<Program> = calculator1::ProgramParser::new()
        .parse(content.as_str()
        )
        .unwrap();

    // Serialize AST to JSON string
    let json_str = serde_json::to_string(&*ast).unwrap();

    // Write the JSON string to a file
    let mut file = File::create("ast_output.json").unwrap();
    file.write_all(json_str.as_bytes()).unwrap();

    // Optionally, you can print the result to the console as well
    println!("Serialized AST written to file 'ast_output.json'");

    let ret = evaluate_program(*ast, false);

    println!("Result: {}", ret);
}
