use std::any::Any;
use lalrpop_util::lalrpop_mod;
use typechecker::typecheck_demo;
use program_ast::*;
// Import the ast module
mod typechecker;

mod program_ast;

lalrpop_mod!(pub calculator1);

fn main() {
    calculator4();
    return;
}


fn typecheck_program(ast: Program) -> i16 {
    match ast {

    }
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

}
