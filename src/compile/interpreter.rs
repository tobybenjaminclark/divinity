use std::any::Any;
use std::collections::HashMap;
use crate::ast::{Block, Expr, Opcode, Program, Statement, TypedArgument};


#[derive(Debug, Clone)]
enum Var {
    Int(i32),
    Float(f64)
}
type SymbolTable = HashMap<String, Var>;


pub fn evaluate_program(program: Program) -> Box<dyn Any> {
    // Create a symbol table for variables
    let mut symbol_table: SymbolTable = HashMap::new();

    // get function names
    let functions = get_function_names(program);
    println!("functions: {:?}", functions);

    // get main
    if let Some(declaration) = functions.get("main") {
        let ret = evaluate_block(declaration, symbol_table);
        return Box::from(ret);
    }


    return Box::from(1);
}

// evaluate blocks
fn evaluate_block(block: &Block, symbol_table: SymbolTable) {

    match block {
        Block::FunctionDefinition(name, vars, ret, body) => {
            evaluate_function(block, &vec![], symbol_table)
        }
        Block::TypeDefinition(_, _, _) => {}
    }
}

fn evaluate_function(function: &Block, args: &Vec<&Expr>, symbol_table: SymbolTable) {
        // create a symbol table with vars
        let mut symbol_table: SymbolTable = HashMap::new();

        match function {
            Block::FunctionDefinition(name, params, ret, body) => {
                if args.len() != params.len() {panic!("args and params have different lengths")}

                for (index, param) in params.iter().enumerate() {
                    match param {
                        TypedArgument::TypedArgument(name, typ) => {
                            symbol_table.insert(name.to_string(), evaluate_expression(args[index]));
                        }
                    }
                }

                for var in args {
                    // insert the called var value in its corresponding position in the symbol table
                    symbol_table.insert(String::from(name), evaluate_expression(var));
                }
            }
            _ => {panic!("expected function")}
        }
}

fn evaluate_function_body(body: &Vec<Statement>, symbol_table: &SymbolTable) {
    for statement in body {
        evaluate_statement(statement, symbol_table);
    }
}

fn evaluate_statement(statement: &Statement, symbol_table: &SymbolTable) {
    match statement {
        Statement::Assignment(name, expr) => {}
        Statement::TypeAssignment(name, val) => {}
        Statement::Expr(expr) => {
        }
    }
}

fn evaluate_expression(expr: &Expr) -> Var {
    match expr {
        Expr::Number(n) => {Var::Int(*n)}
        Expr::Op(l, op, r) => {Var::Int(0)}
        Expr::FunctionCall(name, params) => {Var::Int(0)}
        Expr::Conditional(i, t, e) => {Var::Int(0)}
        Expr::Identifier(name) => {Var::Int(0)}
    }
}

// create a hashmap mapping function names to their definitions
fn get_function_names(program: Program) -> HashMap<String, Block> {
    let mut function_map = HashMap::new();

    if let Program::Program(blocks) = program {
        for block in blocks {
            if let Block::FunctionDefinition(ref name, _, _, _) = block {
                function_map.insert(String::from(name), block);
            }
        }
    }
    function_map
}