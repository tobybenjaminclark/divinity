use std::any::Any;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};
use z3::{Config, Context, Solver, ast};
use z3::ast::Ast;
use crate::program_ast::Block::FunctionDefinition;
use crate::program_ast::{Block, Expr, Opcode, Statement};
use crate::program_ast::TypedArgument::TypedArgument;
use crate::program_ast;



#[derive(Debug)]
enum Z3Value<'ctx> {
    Int(ast::Int<'ctx>),
}



fn expression_inference<'a>(expression: Expr, symbol_table: &'a HashMap<String, Z3Value<'a>>, ctx: &'a Context) -> Z3Value<'a> {
    match expression {
        Expr::Number(num) => {
            Z3Value::Int(ast::Int::from_i64(ctx, num as i64))
        }
        Expr::Op(_, _, _) => {
            Z3Value::Int(ast::Int::from_i64(ctx, 0 as i64))
        }
        Expr::FunctionCall(_, _) => {
            Z3Value::Int(ast::Int::from_i64(ctx, 0 as i64))
        }
        Expr::Conditional(_, _, _) => {
            Z3Value::Int(ast::Int::from_i64(ctx, 0 as i64))
        }
        Expr::Identifier(_) => {
            Z3Value::Int(ast::Int::from_i64(ctx, 0 as i64))
        }
    }
}



#[derive(Clone, Debug)]
pub struct TypeInstance {
    pub(crate) identifier: String, // type name
    pub(crate) args: Vec<crate::program_ast::TypedArgument>, // list of param names to their types
    pub(crate) refinements: Vec<Box<Expr>>,
    pub(crate) base_type: String,
    pub(crate) internal_identifier: String
}



pub fn add_negative_type_constraint<'a>(type_inst: TypeInstance, symb_table: &'a HashMap<String, Z3Value<'a>>, ctx: Context, slvr: Solver) -> () {

    // Destructure the type_inst
    let TypeInstance {
        identifier,
        args,
        refinements,
        base_type,
        internal_identifier,
    } = type_inst;

    for refinement in refinements {
        convert_expr_to_z3(refinement, &symb_table, &ctx, &slvr);
    }
}

fn convert_expr_to_z3<'a>(expr: Box<Expr>, symb_table: &HashMap<String, Z3Value<'a>>, ctx: &Context, solver: &Solver) -> ast::Int {
    match expr.as_ref() {
        Expr::Number(n) => ast::Int::from_i64(ctx, *n as i64),
        Expr::Op(lhs, op, rhs) => {
            let left = convert_expr_to_z3(Box::from(*lhs.clone()), symb_table, ctx, solver);
            let right = convert_expr_to_z3(Box::from(*rhs.clone()), symb_table, ctx, solver);
            match op {
                Opcode::Add => left.add(&right),
                Opcode::Sub => left.sub(&right),
                Opcode::Mul => left.mul(&right),
                Opcode::Div => left.div(&right),
                // For comparison operators, create a new variable that represents the condition
                Opcode::Gt => {
                    // Create a fresh integer variable to store the result of the comparison
                    let result = ast::Int::new_const(ctx, "gt_result");
                    let comparison = left.gt(&right);
                    solver.assert(&comparison.implies(&result._eq(&ast::Int::from_i64(ctx, 1))));
                    solver.assert(&comparison.not().implies(&result._eq(&ast::Int::from_i64(ctx, 0))));
                    result
                }
                Opcode::Lt => {
                    let result = ast::Int::new_const(ctx, "lt_result");
                    let comparison = left.lt(&right);
                    solver.assert(&comparison.implies(&result._eq(&ast::Int::from_i64(ctx, 1))));
                    solver.assert(&comparison.not().implies(&result._eq(&ast::Int::from_i64(ctx, 0))));
                    result
                }
                Opcode::Gteq => {
                    let result = ast::Int::new_const(ctx, "gteq_result");
                    let comparison = left.ge(&right);
                    solver.assert(&comparison.implies(&result._eq(&ast::Int::from_i64(ctx, 1))));
                    solver.assert(&comparison.not().implies(&result._eq(&ast::Int::from_i64(ctx, 0))));
                    result
                }
                Opcode::Lteq => {
                    let result = ast::Int::new_const(ctx, "lteq_result");
                    let comparison = left.le(&right);
                    solver.assert(&comparison.implies(&result._eq(&ast::Int::from_i64(ctx, 1))));
                    solver.assert(&comparison.not().implies(&result._eq(&ast::Int::from_i64(ctx, 0))));
                    result
                }
            }
        }
        Expr::Identifier(id) => {
            // Assuming identifiers map to variables in symb_table
            symb_table.get(id).map(|z3_val| z3_val.to_expr()).unwrap_or_else(|| ctx.empty())
        }
        Expr::FunctionCall(_, _) => {
            // You'd need to handle function calls, possibly by translating them into Z3 expressions
            ctx.empty() // Placeholder for now
        }
        Expr::Conditional(cond, then_expr, else_expr) => {
            let condition = convert_expr_to_z3(cond, symb_table, ctx, solver);
            let then_expr = convert_expr_to_z3(then_expr, symb_table, ctx, solver);
            let else_expr = convert_expr_to_z3(else_expr, symb_table, ctx, solver);
            ctx.if_then_else(&condition, &then_expr, &else_expr)
        }
    }
}


pub fn typecheck_function(function_definition: Block, types: HashMap<String, TypeInstance>, ctx: Context) -> Result<String, String> {
    if let program_ast::Block::FunctionDefinition(name, args, return_type, body) = function_definition {

        // Symbol Table: HashMap to store variable names and their corresponding Z3 values
        let mut type_table: HashMap<String, String> = HashMap::new();
        let mut sym_table: HashMap<String, Z3Value> = HashMap::new();
        let solver = Solver::new(&ctx);

        for arg in args {
            match arg {
                TypedArgument(iden, typ) => {
                    let x = ast::Int::new_const(&ctx, iden.clone());
                    let _ = sym_table.insert(iden.clone(), Z3Value::Int(x));

                    type_table.insert(iden, typ);
                }
            }
        }

        // Process the function body
        for stmnt in body {

            /* Let's add the program! */
            match stmnt {
                Statement::Assignment(variable_name, expression) => {
                    // Create a new Z3 integer variable with the given name
                    let x = ast::Int::new_const(&ctx, variable_name.clone());
                    // Insert the integer variable into the symbol table as Z3Value::Int
                    let _ = sym_table.insert(variable_name.clone(), Z3Value::Int(x));

                    let expression_var = expression_inference(*expression, &sym_table, &ctx);

                    // Now match the symbol table value as Z3Value::Int
                    if let Some(Z3Value::Int(int_x)) = sym_table.get(&variable_name) {
                        if let Z3Value::Int(int_y) = expression_var {
                            // Create an equality constraint
                            let equality_constraint = int_x._eq(&int_y);
                            solver.assert(&equality_constraint);
                        }
                    } else {
                        // This branch should never be reached if x is always a Z3Value::Int
                        panic!("Unexpected Z3Value variant in symbol table");
                    }
                }

                // Handle other statement types (if any)
                Statement::TypeAssignment(name, typ) => {
                    // Check if typ exists in the `types` HashMap
                    if !types.contains_key(&typ) {
                        return Err(format!("Type doesn't exist: {}", typ)); // Return error if type is not found in `types`
                    }

                    // Ensure that name:typ doesn't already exist in the `type_table`
                    if type_table.contains_key(&name) {
                        return Err(format!("Variable {} has already been assigned a type ({})", name, typ));
                    }

                    // If all checks pass, insert the name:typ pair into the type_table
                    type_table.insert(name.clone(), typ.clone());
                }
                _ => {}
            }
        }

        // Iterating over the HashMap
        for (key, value) in &sym_table {
            println!("Iden: {}, Value: {:?}", key, value);
        }

        // Iterating over the HashMap
        for (key, value) in &type_table {
            // Check if the value is defined in the 'types' HashMap
            match types.get(value) {
                Some(_) => {
                    // Value exists in types, proceed with the next item
                    println!("Type '{}' is defined for key '{}'.", value, key);

                }
                None => {
                    // Value is not defined in types, return an error
                    return Err(format!("Type '{}' not defined for key '{}'", value, key));
                }
            }
        }


        /* Check validity. */
        match solver.check() {
            z3::SatResult::Sat => {
                println!("We found a contradiction in {}", name);
            }
            z3::SatResult::Unsat => {
                println!("The {} function is valid.", name);
            }
            z3::SatResult::Unknown => {
                println!("The satisfiability of the inequality is unknown.");
            }
        }
    }
    return Ok("Function has passed.".parse().unwrap())
}



pub fn typecheck_demo() {

    // Create some config and a solver to work in.
    let mut config = Config::new();
    let ctx = Context::new(&config);

    let add_function = Block::FunctionDefinition(
        "add".to_string(),
        vec![
            TypedArgument("a".to_string(), "i32".to_string()),
            TypedArgument("b".to_string(), "i32".to_string()),
        ],
        "i32".to_string(),
        vec![
            Statement::Assignment(
                "result".to_string(),
                Box::new(Expr::Number(10i32)),
            ),
            Statement::Expr(Expr::Identifier("result".to_string())), // Return the result
        ],
    );

    let mut types = HashMap::new();
    typecheck_function(add_function, types, ctx);

}