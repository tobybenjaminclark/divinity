use std::any::Any;
use std::fs::File;
use std::io::{self, Read};
use crate::program_ast::*;
use lalrpop_util::lalrpop_mod;
mod typechecker;
mod program_ast;

lalrpop_mod!(pub calculator1);

fn main() {
    calculator4();
    return;
}

fn calculator4() {
    let program_code = read_file("program.di").expect("Failed to read the file");

    let ast: Box<Program> = calculator1::ProgramParser::new()
        .parse(&program_code)
        .unwrap();

    println!("AST: {:#?}", ast);

}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

