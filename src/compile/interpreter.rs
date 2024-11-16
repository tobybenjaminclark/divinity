use std::any::Any;
use std::collections::HashMap;
use crate::ast::{Block, Expr, Opcode, Program, Statement, TypedArgument};


#[derive(Debug, Clone)]
enum Var {
    Int(i64),
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
        let ret = evaluate_block(declaration, &functions);
        return Box::from(ret);
    }


    return Box::from(1);
}

// evaluate function blocks
fn evaluate_block(block: &Block, functions: &HashMap<String, Block>) {

    match block {
        Block::FunctionDefinition(name, vars, ret, body) => {

            // create a symbol table with vars
            let mut symbol_table: SymbolTable = HashMap::new();
            for var in vars {
                if let TypedArgument::TypedArgument(name, _) = var {
                    symbol_table.insert(String::from(name), Var::Int(0));
                }
            }
            println!("symbol table: {:?}", &symbol_table);

            // evaluate the body with evaluate_function_body(body, symbol table)
        }
        Block::TypeDefinition(_, _, _) => {}
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

fn evaluate_expression(expr: &Expr) {
    match expr {
        Expr::Number(n) => {}
        Expr::Op(l, op, r) => {}
        Expr::FunctionCall(name, params) => {}
        Expr::Conditional(i, t, e) => {}
        Expr::Identifier(name) => {}
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