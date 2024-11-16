use z3::{Config, Context, Solver, ast};

fn main() {
    // Step 1: Create a Z3 configuration and context
    let mut config = Config::new();
    let ctx = Context::new(&config);

    // Step 2: Create a solver
    let solver = Solver::new(&ctx);

    // Step 3: Define variables
    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");

    // Step 4: Assert an inequality (e.g., x < y)
    let inequality = x.lt(&y);
    solver.assert(&inequality);

    // Step 5: Check satisfiability
    match solver.check() {
        z3::SatResult::Sat => {
            println!("The inequality is satisfiable.");
            if let Some(model) = solver.get_model() {
                println!("Model: x = {}, y = {}", model.eval(&x, true).unwrap(), model.eval(&y, true).unwrap());
            }
        }
        z3::SatResult::Unsat => {
            println!("The inequality is not satisfiable.");
        }
        z3::SatResult::Unknown => {
            println!("The satisfiability of the inequality is unknown.");
        }
    }
}
