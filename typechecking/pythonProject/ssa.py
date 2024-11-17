
def replace_identifier(data, source_identifier, new_identifier):
    if isinstance(data, dict):
        return {
            k: (new_identifier if k == "Identifier" and v == source_identifier else replace_identifier(v, source_identifier, new_identifier))
            for k, v in data.items()
        }
    elif isinstance(data, list):
        return [replace_identifier(x, source_identifier, new_identifier) for x in data]
    else:
        return new_identifier if data == source_identifier else data




def ssa_convert(function):
    params = function[1]
    statements = function[3]
    types = {}
    new_types = {}

    for param in params:
        iden = param["TypedArgument"][0]
        typ = param["TypedArgument"][1]
        types[iden] = typ

    index = 0
    while index < len(statements):
        statement = statements[index]
        match list(statement.keys())[0]:
            case "TypeAssignment":
                iden = statement["TypeAssignment"][0]
                typ = statement["TypeAssignment"][1]
                types[iden] = typ

            case "Assignment":
                initial_identifier = statement["Assignment"][0]
                new_identifier = "_" + initial_identifier
                statement["Assignment"][0] = new_identifier

                types[new_identifier] = types[initial_identifier]

                # Replace the identifier in the remaining statements
                statements[index:] = [
                    replace_identifier(s, initial_identifier, new_identifier)
                    for s in statements[index:]
                ]

                # Insert the test statement before the current statement
                temp_statement = {"TypeAssignment":[new_identifier, types[initial_identifier]]}
                statements.insert(index, temp_statement)

                # After inserting, we need to bump the index by 1 to skip over the inserted statement
                index += 1

            case _:
                pass

        # Increment the index to move to the next statement
        index += 1

    return new_types
