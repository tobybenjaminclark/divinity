use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
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
        let ret = evaluate_function(declaration, &vec![], &functions);
        return Box::from(ret);
    }


    return Box::from(1);
}

// evaluate blocks
fn evaluate_block(block: &Block, functions: &HashMap<String, Block>) {

    match block {
        Block::FunctionDefinition(name, vars, ret, body) => {
            // todo: this should not pass in empty parameters
            evaluate_function(block, &vec![], functions);
        }
        Block::TypeDefinition(_, _, _) => {}
    }
}

fn evaluate_function(function: &Block, args: &Vec<Box<Expr>>, functions: &HashMap<String, Block>) -> Var {
        // create a symbol table with vars
        let mut symbol_table: SymbolTable = HashMap::new();

        match function {
            Block::FunctionDefinition(name, params, ret, body) => {
                if args.len() != params.len() {panic!("args and params have different lengths")}

                for (index, param) in params.iter().enumerate() {
                    match param {
                        TypedArgument::TypedArgument(name, typ) => {
                            symbol_table.insert(name.to_string(), evaluate_expression(args[index].clone()));
                        }
                    }
                }

                // evaluate the list of statements in the body
                evaluate_function_body(body, &mut symbol_table, functions)
            }
            _ => {panic!("expected function")}
        }
}

fn evaluate_function_body(body: &Vec<Statement>, symbol_table: &mut SymbolTable, functions: &HashMap<String, Block>) -> Var {
    let return_str = String::from("return");
    for statement in body {
        match statement {
            Statement::Assignment(name, expr) => {
                symbol_table.insert(name.to_string(), evaluate_expression(expr.clone()));
            }
            Statement::TypeAssignment(name, val) => {
                // todo: give name this type
            }
            Statement::Expr(expr) => {
                match expr {
                    Expr::FunctionCall(fname, args) => {
                        if fname == &String::from("return") {
                            if args.len() != 1 {panic!("a function can only return one thing")};
                            return evaluate_expression(args[0].clone()) // todo get the actual type
                        }
                        let function_definition = functions.get(fname).ok_or(format!("function not found: {}", fname)).expect("function not found");
                        evaluate_function(function_definition, args, functions);

                    }
                    _ => panic!("invalid statement")

                }
            }

        }
    }

    Var::Int(0)
}

fn evaluate_expression(expr: Box<Expr>) -> Var {
    match *expr {
        Expr::Number(n) => {Var::Int(n)}
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