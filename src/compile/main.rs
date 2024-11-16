use std::any::Any;
use crate::ast::*;
use lalrpop_util::lalrpop_mod;
use interpreter::*;

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
                q := 1.6747544;
                w := 2;
                e := add(q, w);
                return(e);
            };
            ",
        )
        .unwrap();
    println!("ast: {:#?}", ast);
    let ret = evaluate_program(*ast, true);
    println!("the program returned: {:?}", ret);
}
