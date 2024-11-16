use std::any::Any;
use crate::program_ast::*;
use lalrpop_util::lalrpop_mod;
use typechecker::{typecheck_demo,typecheck_function};
use z3::{Config, Context, Solver, ast};
use z3::ast::Ast;
use std::fs;
use std::io;


// Import the ast module
mod program_ast;
mod interpreter;
mod typechecker;

lalrpop_mod!(pub calculator1);

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "program.div";

    // Read the file content to a String
    let content = fs::read_to_string(file_path)?;

    let ast: Box<Program> = calculator1::ProgramParser::new()
        .parse(&*content)
        .unwrap();
    println!("ast: {:#?}", ast);
    typecheck_demo();

    Ok(())
}

fn typecheck_program(ast: Program) -> i16 {
    match ast {
        Program::Program(blocks) => {
            for block in blocks {
                match block {
                    Block::FunctionDefinition(_, _, _, _) => {
                        // Create some config and a solver to work in.
                        let mut config = Config::new();
                        let ctx = Context::new(&config);

                        typecheck_function(block, ctx);
                    }
                    Block::TypeDefinition(_, _, _) => {}
                }
            }
        }
    }
    return 5 as i16;
}
