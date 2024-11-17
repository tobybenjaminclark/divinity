import tkinter as tk
from tkinter import filedialog
from editor import CodeEditor
from output import OutputPanel
from run_button import run_code  # Import the function for running the code
from divinity_button import divine_code

class CodeEditorApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Code Editor")

        # Set the window size and background color
        self.root.geometry("800x600")
        self.root.configure(bg='#2e2e2e')

        # Set up the frames with custom padding and background colors
        self.editor_frame = CodeEditor(self.root)
        self.editor_frame.grid(row=0, column=0, padx=10, pady=10, sticky="nsew")

        self.output_frame = OutputPanel(self.root)
        self.output_frame.grid(row=1, column=0, padx=10, pady=10, sticky="nsew")

        # Create a frame for the buttons
        self.button_frame = tk.Frame(self.root, bg='#444444')
        self.button_frame.grid(row=2, column=0, padx=10, pady=10, sticky="ew")

        # Add the "Verify" button
        self.verify_button = tk.Button(self.button_frame, text="Verify", command=self.dummy_action,
                                       bg="#6f6f6f", fg="white", font=("Helvetica", 12), relief="flat")
        self.verify_button.grid(row=0, column=0, padx=10, pady=5)

        # Add the "Divinity" button
        self.divinity_button = tk.Button(self.button_frame, text="Divinity", command=lambda:divine_code(self.editor_frame, self.output_frame),
                                        bg="#6f6f6f", fg="white", font=("Helvetica", 12), relief="flat")
        self.divinity_button.grid(row=0, column=1, padx=10, pady=5)

        # Create a "Run Code" button
        self.run_button = tk.Button(self.button_frame, text="Run Code", command=self.run_code,
                                    bg="#ff6347", fg="white", font=("Helvetica", 12, "bold"), relief="flat")
        self.run_button.grid(row=0, column=2, padx=10, pady=5)

        # Create a "Load File" button to open a file dialog for loading a .div file
        self.load_button = tk.Button(self.button_frame, text="Load File", command=self.load_file,
                                     bg="#4682b4", fg="white", font=("Helvetica", 12), relief="flat")
        self.load_button.grid(row=0, column=3, padx=10, pady=5)

        # Make the frames expand dynamically when the window is resized
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_rowconfigure(1, weight=1)
        self.root.grid_columnconfigure(0, weight=1)


    def run_code(self):
        # Use the run_code function imported from run_button.py
        run_code(self.editor_frame, self.output_frame)

    def dummy_action(self):
        return

    def load_file(self):
        # Open a file dialog to select a .div file
        file_path = filedialog.askopenfilename(filetypes=[("DIV Files", "*.div")])

        if file_path:
            try:
                # Read the content of the selected file and display it in the editor
                with open(file_path, 'r') as file:
                    file_content = file.read()
                    self.editor_frame.set_text(file_content)  # Assuming CodeEditor has a set_text method
            except Exception as e:
                print(f"Error loading file: {e}")

def main():
    root = tk.Tk()
    app = CodeEditorApp(root)
    root.mainloop()

if __name__ == "__main__":
    main()
