use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Mul;
use crate::program_ast::{Block, Expr, Opcode, Program, Statement, TypedArgument};


#[derive(Debug, Clone)]
pub enum Var {
    Int(i32),
    Float(f64)
}
type SymbolTable = HashMap<String, Var>;


pub fn evaluate_program(program: Program) -> Var {
    // Create a symbol table for variables
    let mut symbol_table: SymbolTable = HashMap::new();

    // get function names
    let functions = get_function_names(program);
    println!("functions: {:?}", functions);

    // get main
    if let Some(declaration) = functions.get("main") {
        let ret = evaluate_function(declaration, &vec![], &functions);
        println!("main returned {:?}", ret);
        return ret;
    }


    return Var::Int(1);
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
                            let e = evaluate_expression(args[index].clone(), &mut symbol_table, functions);
                            symbol_table.insert(name.to_string(), e);
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
                let e = evaluate_expression(expr.clone(), symbol_table, functions);
                symbol_table.insert(name.to_string(), e.clone());
                println!("inserting {} as {:?}", name, e.clone())
            }
            Statement::TypeAssignment(name, val) => {
                // todo: actually use types
            }
            Statement::Expr(expr) => {
                return evaluate_expression(Box::from(expr.to_owned()), symbol_table, functions);

            }

        }
    }

    Var::Int(0)
}

fn evaluate_function_call(expr: &Expr, symbol_table: &mut SymbolTable, functions: &HashMap<String, Block>) -> Var {
    match expr {
        Expr::FunctionCall(fname, args) => {
            if fname == &String::from("return") {
                if args.len() != 1 { panic!("a function can only return one thing") };
                evaluate_expression(args[0].clone(), symbol_table, functions) // todo get the actual type
            }
            else if fname == &String::from("show") {
                if args.len() != 1 { panic!("a show function can only return one thing") };
                println!("divine stdout: {:?}", args);
                return Var::Int(1);
            } else {
                let function_definition = functions.get(fname).ok_or(format!("function not found: {}", fname)).expect("function not found");
                evaluate_function(function_definition, args, functions)
            }
        },
        _ => panic!("expected function call")
    }
}

fn evaluate_expression(expr: Box<Expr>, symbol_table: &mut SymbolTable, functions:&HashMap<String, Block>) -> Var {
    match *(expr.clone()) {
        Expr::Number(n) => { Var::Int(n) }
        Expr::Op(l, op, r) => { evaluate_op(&*expr, symbol_table, functions) }
        Expr::FunctionCall(name, params) => {
            evaluate_function_call(&*expr, symbol_table, functions)
        }
        Expr::Conditional(i, t, e) =>{
            let condition = evaluate_expression(i, symbol_table, functions);

            // Check if the condition `i >= 1`
            match condition {
                Var::Int(val) => {
                if val >= 1 {
                // Condition is true, evaluate true branch
                evaluate_expression(t, symbol_table, functions)
                } else {
                // Condition is false, evaluate false branch
                evaluate_expression(e, symbol_table, functions)
                }
                }
                _ => panic!("Conditional expression expects an integer value for the condition"),
            }
    }
        Expr::Identifier(name) => {
            // Check if the name exists in the symbol table
            match symbol_table.get(&name) {
                Some(value) => value.to_owned(),
                None => {
                    panic!("Variable '{}' not found in symbol table: {:?}", name, symbol_table); // Gracefully handle missing variable
                }
            }
        }
    }
}

fn evaluate_op(expr: &Expr, symbol_table: &mut SymbolTable, functions: &HashMap<String, Block>) -> Var {
    match expr {
        Expr::Op(left, op, right) => {
            let left_value = evaluate_expression(left.to_owned(), symbol_table, functions);
            let right_value = evaluate_expression(right.to_owned(), symbol_table, functions);

            match (left_value, right_value) {
                (Var::Int(l), Var::Int(r)) => match op {
                    Opcode::Add => Var::Int(l + r),
                    Opcode::Sub => Var::Int(l - r),
                    Opcode::Mul => Var::Int(l * r),
                    Opcode::Div => {
                        if r == 0 {
                            panic!("Division by zero");
                        }
                        Var::Int(l / r)
                    }
                    Opcode::Gt => Var::Int((l > r) as i32),
                    Opcode::Lt => Var::Int((l < r) as i32),
                    Opcode::Gteq => Var::Int((l >= r) as i32),
                    Opcode::Lteq => Var::Int((l <= r) as i32),
                },
                (Var::Float(l), Var::Float(r)) => match op {
                    Opcode::Add => Var::Float(l + r),
                    Opcode::Sub => Var::Float(l - r),
                    Opcode::Mul => Var::Float(l * r),
                    Opcode::Div => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        Var::Float(l / r)
                    }
                    Opcode::Gt => Var::Int((l > r) as i32),
                    Opcode::Lt => Var::Int((l < r) as i32),
                    Opcode::Gteq => Var::Int((l >= r) as i32),
                    Opcode::Lteq => Var::Int((l <= r) as i32),
                },
                (Var::Int(l), Var::Float(r)) => match op {
                    Opcode::Add => Var::Float(l as f64 + r),
                    Opcode::Sub => Var::Float(l as f64 - r),
                    Opcode::Mul => Var::Float(l as f64 * r),
                    Opcode::Div => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        Var::Float(l as f64 / r)
                    }
                    Opcode::Gt => Var::Int((l as f64 > r) as i32),
                    Opcode::Lt => Var::Int((l < r as i32) as i32),
                    Opcode::Gteq => Var::Int((l as f64 >= r) as i32),
                    Opcode::Lteq => Var::Int((l as f64 <= r) as i32),
                },
                (Var::Float(l), Var::Int(r)) => match op {
                    Opcode::Add => Var::Float(l + r as f64),
                    Opcode::Sub => Var::Float(l - r as f64),
                    Opcode::Mul => Var::Float(l * r as f64),
                    Opcode::Div => {
                        if r == 0 {
                            panic!("Division by zero");
                        }
                        Var::Float(l / r as f64)
                    }
                    Opcode::Gt => Var::Int((l > r as f64) as i32),
                    Opcode::Lt => Var::Int((l < r as f64) as i32),
                    Opcode::Gteq => Var::Int((l >= r as f64) as i32),
                    Opcode::Lteq => Var::Int((l <= r as f64) as i32),
                },
            }
        }
        _ => panic!("Expected an operation"),
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