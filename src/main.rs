use std::any::Any;
use crate::program_ast::*;
use lalrpop_util::lalrpop_mod;
use typechecker::typecheck_demo;
use interpreter::*;

// Import the ast module
mod program_ast;
mod interpreter;
mod typechecker;

lalrpop_mod!(pub calculator1);

fn main() {
    calculator4();
    return;
}


fn typecheck_program(ast: Program) -> i16 {
    match ast {
        Program::Program(blocks) => {
            for block in blocks {
                match block {
                    Block::FunctionDefinition(_, _, _, _) => {
                        typecheck_function(block);
                    }
                    Block::TypeDefinition(_, _, _) => {}
                }
            }
        }
    }
    return 5 as i16;
}


fn calculator4() {
    let ast: Box<Program> = calculator1::ProgramParser::new()
        .parse(
            "
            fn add (a:i32, b:i32) -> i32 {
                a + b;
            };

            type newint(c: int) where {
                c > 10,
                c < 15
            };


            fn main () -> i32 {
                q: i32;
                w: i32;
                e: i32;
                q := 1;
                w := 2;
                e := add(q, w);
                return(e);
            };
            ",
        )
        .unwrap();
    println!("ast: {:#?}", ast);

    typecheck_demo();
}
