import z3
from unique import UniqueNameGenerator

# Example usage
names = UniqueNameGenerator()

def build_expr(expression, solver, symbols):
    match list(expression.keys())[0]:
        case "Identifier":
            x = z3.Real(names.generate_name())
            symbol =  symbols[expression["Identifier"]]
            solver.add(x == symbol)
            return x
        case "Number":
            return z3.RealVal(expression["Number"])
        case "Op":
            # First operand (y) and second operand (z)
            x = z3.Real(names.generate_name())
            y = build_expr(expression["Op"][1], solver, symbols)
            z = build_expr(expression["Op"][2], solver, symbols)  # Fixed this to correctly handle both operands

            match expression["Op"][0]:  # Assuming the operator is the first element
                case "Add":
                    solver.add(x == y + z)
                case "Sub":
                    solver.add(x == y - z)
                case "Mul":
                    solver.add(x == y * z)
                case "Div":
                    solver.add(x == y / z)
                case "Gt":
                    solver.add(x == (y > z))
                case "Lt":
                    solver.add(x == (y < z))
                case "Gteq":
                    solver.add(x == (y >= z))
                case "Lteq":
                    solver.add(x == (y <= z))
            return x
        case "FunctionCall":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x
        case "Conditional":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x

def typecheck(function, type_definitions, ssa_types):
    # Initialize the symbol dictionary and valid types list
    symbols = {}
    VALID_TYPES = ([a["TypeDefinition"][0] for a in type_definitions])
    VALID_TYPES.append("i32")

    local_types = ssa_types
    solver = z3.Solver()

    # Print the function details for debugging
    print(f"Function details: {function}")

    name = function[0]
    params = function[1]
    ret_type = function[2]
    statements = function[3]

    # Print the function name and return type for debugging
    print(f"Checking function: {name}, return type: {ret_type}")

    # Process each parameter and add type information to the local types dictionary
    for param in params:
        iden = param["TypedArgument"][0]
        typ = param["TypedArgument"][1]
        print(f"Processing parameter: {iden}, type: {typ}")

        if typ not in VALID_TYPES:
            print(f"ERROR: Invalid Type {typ}")
            raise Exception(f"Invalid Type {typ}")
        local_types[iden] = typ

    print(f"Parameters to {name} -> {ret_type}")

    # Debug: print the local types dictionary
    print(f"Local Types: {local_types}")

    # Add each identifier to the symbols dictionary as a Z3 variable
    for k, v in local_types.items():
        symbols[k] = z3.Real(k)
        print(f"Added symbol for {k} with type {v}")

    # Process each statement in the function
    for index, statement in enumerate(statements):
        print(f"Processing statement {index}: {statement}")
        print(f"Symbols:")
        for k,v in symbols.items():
            print(f"    -   {k}: {v}")
        match list(statement.keys())[0]:
            case "TypeAssignment":
                iden = statement["TypeAssignment"][0]
                typ = statement["TypeAssignment"][1]
                print(f"Type assignment: {iden} = {typ}")

                if typ not in VALID_TYPES:
                    print(f"ERROR: Invalid Type {typ}")
                    raise Exception(f"Invalid Type {typ}")
                # Create a new Z3 variable for the identifier
                symbols[iden] = z3.Real(iden)
                print(f"Added symbol for {iden} as Z3 Real")

            case "Assignment":
                iden = statement["Assignment"][0]
                expr = statement["Assignment"][1]
                print(f"Assignment: {iden} = {expr}")

                # Enforce type compatibility for the assignment.
                expr_var = build_expr(expr, solver, symbols)
                print(f"Built expression for {iden}: {expr_var}")

                solver.add(symbols[iden] == expr_var)
                print(f"Added constraint: {iden} == {expr_var}")

    # Check the solver for a solution
    print("Checking solver...")
    if solver.check() == z3.sat:
        model = solver.model()
        print("Solution found!")
        print(f"Model: {model}")
    else:
        print("No solution found.")

