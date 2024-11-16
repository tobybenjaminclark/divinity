use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Program {
    Program(Vec<Block>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Block {
    FunctionDefinition(
        String,
        Vec<TypedArgument>,
        String,
        Vec<Statement>,
    ),
    TypeDefinition(
        String, // type name
        Vec<TypedArgument>, // list of param names to their types
        Vec<Box<Expr>>, // list of refinements
    ),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TypedArgument {
    TypedArgument(String, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Statement {
    Assignment(String, Box<Expr>),
    TypeAssignment(String, String),
    Expr(Expr),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Expr {
    Number(i32),
    Float(f64),
    Op(Box<Expr>, Opcode, Box<Expr>),
    FunctionCall(String, Vec<Box<Expr>>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
    Identifier(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Gt,
    Lt,
    Gteq,
    Lteq,
}

// Function to convert an instance of Program to JSON string
fn to_json(program: &Program) -> Result<String, serde_json::Error> {
    serde_json::to_string(program)
}

fn main() {
    let example_program = Program::Program(vec![
        Block::FunctionDefinition(
            "main".to_string(),
            vec![TypedArgument::TypedArgument("x".to_string(), "i32".to_string())],
            "i32".to_string(),
            vec![
                Statement::Assignment(
                    "result".to_string(),
                    Box::new(Expr::Op(
                        Box::new(Expr::Number(5)),
                        Opcode::Add,
                        Box::new(Expr::Number(3)),
                    )),
                ),
            ],
        ),
    ]);

    // Serialize the program instance to JSON
    match to_json(&example_program) {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
