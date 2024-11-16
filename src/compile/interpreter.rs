use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Mul;
use crate::ast::{Block, Expr, Opcode, Program, Statement, TypedArgument};


#[derive(Debug, Clone)]
pub enum Var {
    Int(i32),
    Float(f64)
}
type SymbolTable = HashMap<String, Var>;

pub fn evaluate_program(program: Program, debug: bool) -> Var {
    // Get function names
    let functions = get_function_names(&program);
    if debug {
        println!("Available functions: {:?}", functions.keys());
    }

    // Find and evaluate the main function
    if let Some(main_func) = functions.get("main") {
        let ret = evaluate_function(main_func, &vec![], &functions, debug);
        if debug {
            println!("[DEBUG] 'main' function returned: {:?}", ret);
        }
        return ret;
    }

    panic!("No 'main' function found in the program");
}

// Evaluate blocks (cleaned up to show debug flag usage)
fn evaluate_block(block: &Block, functions: &HashMap<String, Block>, debug: bool) {
    match block {
        Block::FunctionDefinition(name, _, _, _) => {
            if debug {
                println!("[DEBUG] Evaluating function: {}", name);
            }
        }
        Block::TypeDefinition(_, _, _) => {
            if debug {
                println!("[DEBUG] Skipping type definition block");
            }
        }
    }
}

fn evaluate_function(
    function: &Block,
    args: &Vec<Box<Expr>>,
    functions: &HashMap<String, Block>,
    debug: bool,
) -> Var {
    let mut symbol_table: SymbolTable = HashMap::new();

    if debug {
        println!("[DEBUG] Evaluating function with args: {:?}", args);
    }

    match function {
        Block::FunctionDefinition(name, params, _, body) => {
            if args.len() != params.len() {
                panic!(
                    "Argument count mismatch for function '{}'. Expected {}, got {}",
                    name,
                    params.len(),
                    args.len()
                );
            }

            for (index, param) in params.iter().enumerate() {
                if let TypedArgument::TypedArgument(param_name, _) = param {
                    let evaluated_arg = evaluate_expression(args[index].clone(), &mut symbol_table, functions, debug);
                    symbol_table.insert(param_name.to_string(), evaluated_arg);
                }
            }

            evaluate_function_body(body, &mut symbol_table, functions, debug)
        }
        _ => panic!("Expected a function block"),
    }
}

fn evaluate_function_body(
    body: &Vec<Statement>,
    symbol_table: &mut SymbolTable,
    functions: &HashMap<String, Block>,
    debug: bool,
) -> Var {
    for statement in body {
        match statement {
            Statement::Assignment(name, expr) => {
                let value = evaluate_expression(expr.clone(), symbol_table, functions, debug);
                if debug {
                    println!("[DEBUG] Assigning {} = {:?}", name, value);
                }
                symbol_table.insert(name.clone(), value);
            }
            Statement::TypeAssignment(name, _) => {
                if debug {
                    println!("[DEBUG] Skipping type assignment for {}", name);
                }
            }
            Statement::Expr(expr) => {
                if debug {
                    println!("[DEBUG] Evaluating expression: {:?}", expr);
                }
                return evaluate_expression(Box::new(expr.clone()), symbol_table, functions, debug);
            }
        }
    }

    Var::Int(0)
}

fn evaluate_expression(
    expr: Box<Expr>,
    symbol_table: &mut SymbolTable,
    functions: &HashMap<String, Block>,
    debug: bool,
) -> Var {
    match *expr {
        Expr::Number(n) => Var::Int(n),
        Expr::Float(n) => Var::Float(n),
        Expr::Op(_, _, _) => evaluate_op(&*expr, symbol_table, functions, debug),
        Expr::FunctionCall(_, _) => evaluate_function_call(&*expr, symbol_table, functions, debug),
        Expr::Conditional(cond, if_branch, else_branch) => {
            let condition = evaluate_expression(cond, symbol_table, functions, debug);
            match condition {
                Var::Int(val) if val != 0 => evaluate_expression(if_branch, symbol_table, functions, debug),
                Var::Int(_) => evaluate_expression(else_branch, symbol_table, functions, debug),
                _ => panic!("Condition must evaluate to an integer"),
            }
        }
        Expr::Identifier(name) => symbol_table
            .get(&name)
            .cloned()
            .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),
    }
}

fn evaluate_op(
    expr: &Expr,
    symbol_table: &mut SymbolTable,
    functions: &HashMap<String, Block>,
    debug: bool,
) -> Var {
    // Evaluate operation with improved error handling and cleaner debug
    if let Expr::Op(left, op, right) = expr {
        let left_value = evaluate_expression(left.clone(), symbol_table, functions, debug);
        let right_value = evaluate_expression(right.clone(), symbol_table, functions, debug);

        if debug {
            println!("[DEBUG] Performing {:?} {:?} {:?}", left_value.clone(), op, right_value.clone());
        }

        match (left_value.clone(), right_value.clone()) {
            // Integer operations
            (Var::Int(l), Var::Int(r)) => match op {
                Opcode::Add => Var::Int(l + r),
                Opcode::Sub => Var::Int(l - r),
                Opcode::Mul => Var::Int(l * r),
                Opcode::Div if r != 0 => Var::Int(l / r),
                Opcode::Div => panic!("Division by zero"),
                _ => panic!("Unsupported operation for integers"),
            },
            // Float operations
            (Var::Float(l), Var::Float(r)) => match op {
                Opcode::Add => Var::Float(l + r),
                Opcode::Sub => Var::Float(l - r),
                Opcode::Mul => Var::Float(l * r),
                Opcode::Div if r != 0.0 => Var::Float(l / r),
                Opcode::Div => panic!("Division by zero"),
                _ => panic!("Unsupported operation for floats"),
            },
            // Mixed operations (Int + Float or Float + Int)
            (Var::Int(l), Var::Float(r)) => match op {
                Opcode::Add => Var::Float(l as f64 + r),
                Opcode::Sub => Var::Float(l as f64 - r),
                Opcode::Mul => Var::Float(l as f64 * r),
                Opcode::Div if r != 0.0 => Var::Float(l as f64 / r),
                Opcode::Div => panic!("Division by zero"),
                _ => panic!("Unsupported operation for mixed types"),
            },
            (Var::Float(l), Var::Int(r)) => match op {
                Opcode::Add => Var::Float(l + r as f64),
                Opcode::Sub => Var::Float(l - r as f64),
                Opcode::Mul => Var::Float(l * r as f64),
                Opcode::Div if r != 0 => Var::Float(l / r as f64),
                Opcode::Div => panic!("Division by zero"),
                _ => panic!("Unsupported operation for mixed types"),
            },
            // Unsupported operand types
            _ => panic!("Unsupported operand types: {:?} and {:?}", left_value, right_value.clone()),
        }
    } else {
        panic!("Expected an operation expression");
    }
}

// Pass debug flag to helper functions
fn evaluate_function_call(expr: &Expr, symbol_table: &mut SymbolTable, functions: &HashMap<String, Block>, debug: bool) -> Var {
    match expr {
        Expr::FunctionCall(fname, args) => {
            if fname == "return" {
                if args.len() != 1 {
                    panic!("A function can only return one value");
                }
                evaluate_expression(args[0].clone(), symbol_table, functions, debug)
            } else if fname == "show" {
                if args.len() != 1 {
                    panic!("A show function can only accept one argument");
                }
                let value = evaluate_expression(args[0].clone(), symbol_table, functions, debug);
                println!("divine stdout: {:?}", value);
                return Var::Int(1);
            } else {
                // Retrieve the function definition
                let function_definition = functions
                    .get(fname)
                    .unwrap_or_else(|| panic!("Function not found: {}", fname));

                // Evaluate the arguments in the current symbol table context
                let args_values: Vec<Box<Expr>> = args
                    .iter()
                    .map(|arg| {
                        let evaluated_value = evaluate_expression(arg.clone(), symbol_table, functions, debug);
                        match evaluated_value {
                            Var::Int(n) => Box::new(Expr::Number(n)),
                            Var::Float(f) => Box::new(Expr::Float(f)),
                            _ => panic!("Expected Int or Float"),
                        }
                    })
                    .collect();


                println!(
                    "Calling function '{}' with evaluated args: {:?}",
                    fname, args_values
                );

                // Evaluate the function with the evaluated arguments
                evaluate_function(function_definition, &args_values, functions, debug)
            }
        }
        _ => panic!("Expected a function call"),
    }
}
fn get_function_names(program: &Program) -> HashMap<String, Block> {
    let mut function_map = HashMap::new();
    if let Program::Program(blocks) = program {
        for block in blocks {
            if let Block::FunctionDefinition(ref name, _, _, _) = block {
                function_map.insert(name.clone(), block.clone());
            }
        }
    }
    function_map
}
