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
                return(a + b);
            };

            type newint(c: i32) where {
                c > 10,
                c < 15
            };

            type newint2(c: newint) where {
                c > 10,
                c < 15
            };

            fn main () -> i32 {
                q: newint2;
                w: i32;
                e: i32;
                getint(r);
                show(r);
                q := 1.6747544;
                w := 2;
                e := add(q, w);
                return(e);
            };
            ",
        )
        .unwrap();
    let ret = evaluate_program(*ast, false);
    println!("Result: {}", ret);
}
