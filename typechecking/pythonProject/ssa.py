
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
    statements = function[3]

    for index, statement in enumerate(statements):
        match list(statement.keys())[0]:
            case "Assignment":
                initial_identifier = statement["Assignment"][0]
                new_identifier = "_" + initial_identifier
                statement["Assignment"][0] = new_identifier
                # Replace the identifier in the remaining statements
                statements[index:] = [
                    replace_identifier(s, initial_identifier, new_identifier)
                    for s in statements[index:]
                ]
            case _:
                pass
        print(statement)
