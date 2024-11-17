import z3
from unique import UniqueNameGenerator
import types

# Example usage
names = UniqueNameGenerator()

def build_type_expr(expression, solver, symbols, sub_source, sub_target):
    if expression is None:
        return None

    # Check if expression is a lambda function
    if isinstance(expression, types.LambdaType):
        # Do something if it's a lambda
        print("The expression is a lambda function.")
        return expression

    match list(expression.keys())[0]:
        case "Identifier":
            if expression["Identifier"] == sub_source:
                return sub_target
            else:
                x = z3.Real(names.generate_name())
                symbol = symbols[expression["Identifier"]]
                solver.add(x == symbol)
                return x
        case "Number":
            return z3.RealVal(expression["Number"])
        case "Op":
            x = z3.Real(names.generate_name())
            y = build_type_expr(expression["Op"][0], solver, symbols, sub_source, sub_target)
            z = build_type_expr(expression["Op"][2], solver, symbols, sub_source, sub_target)

            match expression["Op"][1]:
                case "Add":
                    solver.add(x == y + z)
                case "Sub":
                    solver.add(x == y - z)
                case "Mul":
                    solver.add(x == y * z)
                case "Div":
                    solver.add(x == y / z)
                case "Gt":
                    return lambda: y > z
                case "Lt":
                    return lambda: y < z
                case "Gteq":
                    return lambda: y >= z
                case "Lteq":
                    return lambda: y <= z
            return None
        case "FunctionCall":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x
        case "Conditional":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x

def build_expr(expression, solver, symbols):
    match list(expression.keys())[0]:
        case "Identifier":
            x = z3.Real(names.generate_name())
            symbol = symbols[expression["Identifier"]]
            solver.add(x == symbol)
            return x
        case "Number":
            x = z3.Real(names.generate_name())
            solver.add(x == expression["Number"])
            return x
        case "Op":
            x = z3.Real(names.generate_name())
            try:
                y = build_expr(expression["Op"][0], solver, symbols)
                z = build_expr(expression["Op"][2], solver, symbols)
            except Exception as e:
                print(expression)
                raise e
            try:
                match expression["Op"][1]:
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
            except Exception as e:
                print(expression)
                raise e
        case "FunctionCall":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x
        case "Conditional":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x
        case _:
            print(expression)
            raise Exception()


def typecheck(function, type_definitions, ssa_types):
    symbols = {}
    VALID_TYPES = ([a["TypeDefinition"][0] for a in type_definitions])
    VALID_TYPES.append("i32")

    real_local_types = {}
    local_types = ssa_types
    solver = z3.Solver()

    fname = function[0]
    params = function[1]
    ret_type = function[2]
    statements = function[3]
    total_refinements = 0;
    param_idens = []

    for param in params:
        iden = param["TypedArgument"][0]
        param_idens.append(iden)
        typ = param["TypedArgument"][1]

        if typ not in VALID_TYPES:
            raise Exception(f"Invalid Type {typ}")
        local_types[iden] = typ
        real_local_types[iden] = typ

    for k, v in local_types.items():
        symbols[k] = z3.Real(k)

    for index, statement in enumerate(statements):
        print(fname)
        print(f" -> {statement}")
        match list(statement.keys())[0]:
            case "TypeAssignment":
                iden = statement["TypeAssignment"][0]
                typ = statement["TypeAssignment"][1]

                real_local_types[iden] = typ
                if typ not in VALID_TYPES:
                    raise Exception(f"Invalid Type {typ}")
                symbols[iden] = z3.Real(iden)

            case "Assignment":
                iden = statement["Assignment"][0]
                expr = statement["Assignment"][1]

                expr_var = build_expr(expr, solver, symbols)
                solver.add(symbols[iden] == expr_var)

            case _:
                pass

    # Ok now we need to add the negative categories for the given types
    for k, v in symbols.items():
        t = real_local_types[k]
        #print(f"{k} - {v} - {t}")

        # ok this is just a number.
        if t == "i32":
            pass

        # ok this is something important.
        # we must construct de negative category!
        else:
            # we must find the corresponding type declaration
            found = False
            for typ in type_definitions:
                name = typ["TypeDefinition"][0]
                if name == t:
                    found = True
                    break
                else:
                    continue

            if not found:
                raise Exception(f"Unknown Type {t}")

            # ok we have found the type definition it is `typ`
            # now we must access it's refinements and build these into z3
            _refinements = typ["TypeDefinition"][2]
            total_refinements += len(_refinements)
            #print(_refinements)
            source = typ["TypeDefinition"][1][0]["TypedArgument"][0]
            #print(source)
            ref = [build_type_expr(refinemnet, solver, symbols, source, symbols[k]) for refinemnet in _refinements]

            if k in param_idens:
                solver.add((z3.And([r() for r in ref])))
            else:
                solver.add(z3.Not((z3.And([r() for r in ref]))))

    if solver.check() == z3.sat:
        if total_refinements == 0:
            print(f"FUNCTION {fname} IS GOOD")
        else:
            print(f"FUNCTION {fname} is BAD!\n")
            model = solver.model()

            # Iterate over all the declarations in the model and print their values
            for d in model.decls():
                value = model[d]
                print(f"    -   {d.name()} = {value}")

            for index, constraint in enumerate(solver.assertions()):
                print(f" {index} :: {constraint}")

    elif solver.check() == z3.unsat:
        print(f"FUNCTION {fname} IS GOOD")
        model = None
