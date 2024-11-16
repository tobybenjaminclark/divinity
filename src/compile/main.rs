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
            fn add (a:i32, c:i32) -> i32 {
                a + b;
            };

            type newint(a: int) where {
                a > 10,
                a < 15
            };


            fn main () -> i32 {
                a: i32;
                b: i32;
                c: i32;
                a := 1;
                b := 2;
                c := add(a, b);
                show(c);
                return(1);
            };
            ",
        )
        .unwrap();
    println!("ast: {:#?}", ast);
    let ret = evaluate_program(*ast);
    print_result(ret);
}

fn print_result(result: Box<dyn Any>) {
    // Attempt to downcast the Box<dyn Any> into a known type
    if let Some(_) = result.downcast_ref::<(i32)>() {
        println!("Result is a unit: ()");
    } else {
        println!("Result is not a unit type.");
    }
}
