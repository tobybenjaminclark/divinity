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
                case "Eq":
                    return lambda: y == z
            return None
        case "FunctionCall":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x
        case "Conditional":
            x = z3.Real(names.generate_name())
            solver.add(x == 10)  # Simplified for now
            return x

def build_expr(expression, solver, symbols, funcs, types):
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
                y = build_expr(expression["Op"][0], solver, symbols, funcs, types)
                z = build_expr(expression["Op"][2], solver, symbols, funcs, types)
            except Exception as e:
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
                    case "Eq":
                        solver.add(x == (y == z))
                return x
            except Exception as e:
                raise e
        case "Conditional":
            lst = expression["Conditional"]
            x = z3.Real(names.generate_name())
            a = build_expr(lst[0], solver, symbols, funcs, types)
            b = build_expr(lst[1], solver, symbols, funcs, types)
            c = build_expr(lst[2], solver, symbols, funcs, types)
            solver.add(x == z3.If(a > 0, b, c))
            return x
        case "FunctionCall":
            # we need to set that x is bound by the return type of the function
            # oh, we also need to check that the input variables are all good.

            # firstly does the function  even exist
            function_name = expression["FunctionCall"][0]

            found = False
            for current_func in funcs:
                if function_name == current_func["FunctionDefinition"][0]:
                    found = True
                    break
                else:
                    continue
            if not found:
                raise Exception(f"Undefined Function {function_name}")

            # we need to do the contravariant covariant type checking shit here.
            # i forgot what this actually does tho because it's 2am
            # ok lets do this now, are we giving the function correct arguments?
            args = current_func["FunctionDefinition"][1]
            if len(expression["FunctionCall"][1]) != len(args):
                raise Exception(f"Function Call to {function_name} has {len(args)} arguments, expected only {len(args)}")

            # now let's verify each argument using a constriant
            for i, a in enumerate(args):
                try:
                    typ_iden = a["TypedArgument"][1]
                except Exception as e:
                    raise e

                if typ_iden == "i32":
                    #most generic type, all space is valid.
                    continue
                # ok now this can be something more specific
                found = False
                try:
                    for typ in types:
                        name = typ["TypeDefinition"][0]
                        if name == typ_iden:
                            found = True
                            break
                        else:
                            continue
                except Exception as e:
                    raise e
                if not found:
                    raise Exception(f"Unknown Type {typ_iden}")
                # ok now we have the type definition, we can check it
                # let's add these refinements to x
                expr_val = build_expr(expression["FunctionCall"][1][i], solver, symbols, funcs, types)
                _refinements = typ["TypeDefinition"][2]
                source = typ["TypeDefinition"][1][0]["TypedArgument"][0]
                var = z3.Real(names.generate_name())
                solver.add(var == expr_val)
                ref = [build_type_expr(refinemnet, solver, symbols, source, var) for refinemnet in _refinements]
                # since this is a negative space, we need to check if something CAN exist in it (sat is FAIL remember)
                solver.add(z3.Not((z3.And([r() for r in ref]))))

            # we have the string return type, now we need to get
            func_ret_type_iden = current_func["FunctionDefinition"][2]

            # great it's just a number
            if func_ret_type_iden == "i32":
                x = z3.Real(names.generate_name())
                return x

            # we must find the corresponding type declaration
            found = False
            try:
                for typ in types:
                    name = typ["TypeDefinition"][0]
                    if name == func_ret_type_iden:
                        found = True
                        break
                    else:
                        continue
            except Exception as e:
                raise e

            if not found:
                raise Exception(f"Unknown Type {t}")

            # ok we have found the type definition it is `typ`
            # now we must access it's refinements and build these into z3
            _refinements = typ["TypeDefinition"][2]

            # let's add these refinements to x
            source = typ["TypeDefinition"][1][0]["TypedArgument"][0]
            var = z3.Real(names.generate_name())
            ref = [build_type_expr(refinemnet, solver, symbols, source, var) for refinemnet in _refinements]
            solver.add((z3.And([r() for r in ref])))
            return var
        case _:
            raise Exception()


def typecheck(function, type_definitions, ssa_types, funcs):
    returns_well = False
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

                expr_var = build_expr(expr, solver, symbols, funcs, type_definitions)
                solver.add(symbols[iden] == expr_var)

            case "Expr":
                expr = statement["Expr"]
                func = expr["FunctionCall"]
                if(len(func[1]) != 1):
                    raise Exception("You can only return 1 thing from a function in Divinity.")
                if(func[0] == "return"):
                    returns_well = True
                    # we know return type is rest_type
                    # if this is i32, then this is the most generic and everything is valid!
                    if ret_type == "i32":
                        continue
                    else:
                        # we must find the corresponding type declaration
                        found = False
                        for typ in type_definitions:
                            name = typ["TypeDefinition"][0]
                            if name == ret_type:
                                found = True
                                break
                            else:
                                continue
                        if not found:
                            raise Exception(f"Unknown Type {ret_type}")
                        # we got the type

                    return_expr = func[1][0]
                    return_expr_var = build_expr(return_expr, solver, symbols, funcs, type_definitions)

                    # Okay we have a return value!
                    # We need to add some sort of constraint to see if the variable can escape this space.
                    var = z3.Real(names.generate_name())
                    _refinements = typ["TypeDefinition"][2]
                    total_refinements += len(_refinements)
                    source = typ["TypeDefinition"][1][0]["TypedArgument"][0]
                    ref = [build_type_expr(refinemnet, solver, symbols, source, var) for refinemnet in
                           _refinements]
                    solver.add(z3.Not((z3.And([r() for r in ref]))))
                    solver.add(return_expr_var == var)
            case _:
                pass

    # Ok now we need to add the negative categories for the given types
    for k, v in symbols.items():
        t = real_local_types[k]

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
        model = solver.model()

        # Iterate over all the declarations in the model and print their values
        print("Model Values")
        for d in model.decls():
            value = model[d]
            print(f" ↪ {d.name()} = {value}")

        print("Constraints")
        for index, constraint in enumerate(solver.assertions()):
            print(f" ↪ {index} :: {constraint}")

        raise Exception(f"Function {fname} is not 100% safe!")


    elif solver.check() == z3.unsat:
        print(f"FUNCTION {fname} IS GOOD")
        model = None
        return True