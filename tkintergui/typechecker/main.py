import z3
import json
from ssa import ssa_convert
from typechecker import typecheck

def run(file_path: str):
    file_path = 'typechecker/program.json'
    try:
        with open(file_path, 'r') as file:
            data = json.load(file)
    except FileNotFoundError:
        print(f"The file {file_path} was not found.")
    except json.JSONDecodeError as e:
        print(f"Error decoding JSON: {e}")

    types = list(filter(lambda x: "TypeDefinition" in x.keys(), data["Program"]))
    functions = list(filter(lambda x: "FunctionDefinition" in x.keys(), data["Program"]))

    for f in functions:
        print(f"Checking {f["FunctionDefinition"][0]}")
        ssa_types = ssa_convert(f["FunctionDefinition"])
        try:
            typecheck(f["FunctionDefinition"], types, ssa_types, functions)
        except Exception as e:
            pass
        print("\n\n")

if __name__ == "__main__":
    run("program.json")





"""
type int50(g: i32) where {
    g == 50,
};

type intUnder40(g: i32) where {
    g < 40,
};

fn test_if_else() -> i32 {
    a: int50;
    a := if 1 then 50 else 0;
    return(12)
};

fn test_if_else_fail() -> i32 {
    a: int50;
    a := if 1 then 0 else 50;
    return(12)
};


fn test_if_else_fail2(a: i32) -> i32 {
    b: int50;
    b := if a then 50 else 49;
    return(12)
};


fn main () -> i32 {
    return(1)
};
"""

