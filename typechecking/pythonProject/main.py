import z3
import json
from ssa import ssa_convert
from typechecker import typecheck

def run(file_path: str):
    file_path = 'program.json'
    try:
        with open(file_path, 'r') as file:
            data = json.load(file)
    except FileNotFoundError:
        print(f"The file {file_path} was not found.")
    except json.JSONDecodeError as e:
        print(f"Error decoding JSON: {e}")

    types = list(filter(lambda x: "TypeDefinition" in x.keys(), data["Program"]))
    functions = list(filter(lambda x: "FunctionDefinition" in x.keys(), data["Program"]))

    for t in types:
        print(t)

    for f in functions:
        ssa_types = ssa_convert(f["FunctionDefinition"])
        typecheck(f["FunctionDefinition"], types, ssa_types)

if __name__ == "__main__":
    run("program.json")


