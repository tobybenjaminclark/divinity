import os
import sys
import z3
import json
from output import OutputPanel, StdoutRedirector
from typechecker.ssa import ssa_convert
from typechecker.typechecker import typecheck

# Function to simulate running the program and capturing stdout
def run(file_path: str, output_panel: OutputPanel):
    lines = ""
    try:
        with open(file_path, 'r') as file:
            data = json.load(file)
            
    except Exception as e:
        return (False, "The program did not compile.")

    #os.remove("ast_output.json")

    types = list(filter(lambda x: "TypeDefinition" in x.keys(), data["Program"]))
    functions = list(filter(lambda x: "FunctionDefinition" in x.keys(), data["Program"]))

    retval = True
    for f in functions:
        lines += f"Checking {f['FunctionDefinition'][0]}\n"
        ssa_types = ssa_convert(f["FunctionDefinition"])
        try:
            success, lines = typecheck(f["FunctionDefinition"], types, ssa_types, functions, lines)
        except Exception as e:
            retval = False
        
    return (retval, lines)

# Function to handle the execution in a thread
def run_in_thread(file_path, output_panel):
    """print("running")
    # Redirect stdout to the output_panel
    redirector = StdoutRedirector(output_panel)
    sys.stdout = redirector"""

    result, message = run(file_path, output_panel)

    # Optionally handle result (e.g., change state of the label)
    output_panel.set_state(result)

    return (result, message)
    

if __name__ == "__main__":

    run("program.json", None)





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

