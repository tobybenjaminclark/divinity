use std::any::Any;
use std::fs;
use lalrpop_util::lalrpop_mod;
use std::fs::File;
use std::io::Write;
use crate::compile::ast::Program;
use crate::compile::interpreter::{evaluate_program, Var};

// Import the ast module
mod compile;

lalrpop_mod!(pub calculator1);

pub fn calculator4(content: String) -> Var {
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

    return ret;
}
