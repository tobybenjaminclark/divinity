# run_button.py
import tkinter as tk
import subprocess

class RunButton(tk.Button):
    def __init__(self, master, editor_frame, output_frame):
        self.editor_frame = editor_frame
        self.output_frame = output_frame
        super().__init__(master, text="Run Code", command=self.run_code)
        
    def run_code(self):
        code = self.editor_frame.get_code()
        if code.strip():
            self.output_frame.clear_output()
            self.execute_code(code)
        else:
            self.output_frame.show_output("Error: No code to run.")
    
    def execute_code(self, code):
        try:
            # For demo purposes, we will run Python code via subprocess
            result = subprocess.run(['python3', '-c', code], capture_output=True, text=True)
            output = result.stdout if result.returncode == 0 else result.stderr
            self.output_frame.show_output(output)
        except Exception as e:
            self.output_frame.show_output(f"Execution error: {str(e)}")
