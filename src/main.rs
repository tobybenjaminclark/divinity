use std::any::Any;
use std::collections::HashMap;
use crate::program_ast::*;
use lalrpop_util::lalrpop_mod;
use typechecker::{typecheck_demo,typecheck_function, TypeInstance};
use z3::{Config, Context, Solver, ast};
use z3::ast::Ast;
use std::fs;
use std::io;
use program_ast::Block;


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
    typecheck_program(*ast);
    Ok(())
}



fn typecheck_program(ast: Program) -> Result<String, String> {

    let mut types: HashMap<String, TypeInstance> = HashMap::new();

    /* Let's find out all of the types that exist! */
    match ast.clone() {
        Program::Program(blocks) => {
            for block in blocks {
                match block {
                    Block::FunctionDefinition(_, _, _, _) => {}
                    Block::TypeDefinition(identifier, args, refinement) => {

                        if let Some(first) = args.get(0) {
                            match first {
                                TypedArgument::TypedArgument(_internal_identifier, _base) => {
                                    let mut base_type = _base.clone();
                                    let mut internal_identifier = _internal_identifier.clone();

                                    let type_def = TypeInstance {
                                        identifier,
                                        args,
                                        refinements: refinement,
                                        base_type: (*base_type).parse().unwrap(),
                                        internal_identifier: (*internal_identifier).parse().unwrap()
                                    };
                                    types.insert(type_def.identifier.clone(), type_def);
                                }
                            }
                        } else {
                            return Err(format!("Type Declaration for {} takes no arguments.", identifier));
                        }
                    }
                }
            }
        }
    }

    match ast {
        Program::Program(blocks) => {
            for block in blocks {
                match block {
                    Block::FunctionDefinition(_, _, _, _) => {
                        // Create some config and a solver to work in.
                        let mut config = Config::new();
                        let ctx = Context::new(&config);

                        typecheck_function(block, types.clone(), ctx);
                    }
                    Block::TypeDefinition(_, _, _) => {}
                }
            }
        }
    }
    return Ok(String::from("Program seems correct."))
}
