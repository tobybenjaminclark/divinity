use std::any::Any;
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
    let ast: Box<Program> = calculator1::ProgramParser::new()
        .parse(
            "
                type int1(g: i32) where {
                    g >= 20,
                    g <= 40
                };

                type int2(c: i32) where {
                    c > 10,
                    c < 20
                };

                fn add (a:int1, b:int1) -> int2 {
                    return(a + b)
                };

                fn ssa_test(a:i32) -> i32 {
                    a1: i32;
                    a2: i32;
                    a3: i32;

                    a := a1;
                    a := a2;
                    a := a3;
                    return(a)
                };

                fn main () -> i64 {
                    a: int2;
                    a := add(15, 15);
                    return(a)
                };
            ",
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
