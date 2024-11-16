use std::collections::HashMap;
use z3::{Config, Context, Solver, ast};
use z3::ast::Ast;
use crate::program_ast::*;

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

pub fn typecheck_function(function_definition: Block, ctx: Context) -> bool {
    if let Block::FunctionDefinition(name, args, return_type, body) = function_definition {

        // Symbol Table: HashMap to store variable names and their corresponding Z3 values
        let mut sym_table: HashMap<String, Z3Value> = HashMap::new();
        let solver = Solver::new(&ctx);

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
                _ => {}
            }
        }

        /* Now the program has been added, let's add type constraints! */
        let a = sym_table.get("result").unwrap();
        match a {
            Z3Value::Int(int_x) => {
                // Now you can use int_x as an ast::Int
                let gt_constraint = int_x.gt(&ast::Int::from_i64(&ctx, 12 as i64));
                let not_gt_constraint = gt_constraint.not(); // Negate the 'greater than' condition
                solver.assert(&not_gt_constraint);
            }
        }

        /* Check validity. */
        match solver.check() {
            z3::SatResult::Sat => {
                println!("We found a contradiction!");
                if let Some(model) = solver.get_model() {
                    let a = sym_table.get("result").unwrap();
                    match a {
                        Z3Value::Int(int_x) => {
                            println!("Model: result = {}", model.eval(int_x, true).unwrap());
                        }
                    }
                }
            }
            z3::SatResult::Unsat => {
                println!("The program is valid.");
            }
            z3::SatResult::Unknown => {
                println!("The satisfiability of the inequality is unknown.");
            }
        }
    }
    true // return true or some result depending on the logic
}





pub fn typecheck_demo() {

    // Create some config and a solver to work in.
    let mut config = Config::new();
    let ctx = Context::new(&config);

    let add_function = Block::FunctionDefinition(
        "add".to_string(),
        vec![
            crate::typechecker::TypedArgument::TypedArgument("a".to_string(), "i32".to_string()),
            crate::typechecker::TypedArgument::TypedArgument("b".to_string(), "i32".to_string()),
        ],
        "i32".to_string(),
        vec![
            Statement::Assignment(
                "result".to_string(),
                Box::new(Expr::Number(10 as i32)),
            ),
            Statement::Expr(Expr::Identifier("result".to_string())), // Return the result
        ],
    );

    typecheck_function(add_function, ctx);

}
