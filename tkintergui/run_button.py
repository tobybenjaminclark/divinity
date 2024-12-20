import os
import tkinter as tk
import subprocess
import re

def run_code(editor_frame, output_frame, rust_exe_path="input/divinity.exe"):
    # Get code from the editor frame (textbox)
    code = editor_frame.get_code()
    if code.strip():
        output_frame.clear_output()
        execute_code(code, output_frame, rust_exe_path)
    else:
        output_frame.show_output("Error: No code to run.")

def execute_code(code, output_frame, rust_exe_path):
    try:
        # Remove comments wrapped in /* */
        code = remove_comments(code)

        # Ensure the Rust executable exists
        if not os.path.exists(rust_exe_path):
            output_frame.show_output(f"Error: {rust_exe_path} not found.")
            return
        
        # Create a temporary file to store the code
        with open("temp_program.div", "w") as temp_file:
            temp_file.write(code)
        
        # Run the compiled Rust binary with subprocess and the temporary file
        _ = subprocess.run([rust_exe_path, "ast", "temp_program.div"], capture_output=True, text=True)
        result = subprocess.run([rust_exe_path, "run", "ast_output.json"], capture_output=True, text=True)
        
        # Capture the output or error
        output = result.stdout if result.returncode == 0 else result.stderr
        output_frame.show_output(output)

        # Optionally delete the temporary file after execution
        os.remove("temp_program.div")

    except Exception as e:
        output_frame.show_output(f"Execution error: {str(e)}")

def remove_comments(code):
    """Remove comments wrapped in /* */."""
    # Regex to match anything between /* and */
    code_without_comments = re.sub(r'/\*.*?\*/', '', code, flags=re.DOTALL)
    return code_without_comments
