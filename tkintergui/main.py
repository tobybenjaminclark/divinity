import tkinter as tk
from editor import CodeEditor
from output import OutputPanel
from run_button import run_code  # Import the function for running the code

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

        # Add some spacing and style the buttons
        self.verify_button = tk.Button(self.button_frame, text="Verify", command=self.dummy_action,
                                       bg="#6f6f6f", fg="white", font=("Helvetica", 12), relief="flat")
        self.verify_button.grid(row=0, column=0, padx=10, pady=5)

        self.divinity_button = tk.Button(self.button_frame, text="Divinity", command=self.dummy_action,
                                        bg="#6f6f6f", fg="white", font=("Helvetica", 12), relief="flat")
        self.divinity_button.grid(row=0, column=1, padx=10, pady=5)

        # Create a styled "Run Code" button that will trigger the run_code function
        self.run_button = tk.Button(self.button_frame, text="Run Code", command=self.run_code,
                                    bg="#ff6347", fg="white", font=("Helvetica", 12, "bold"), relief="flat")
        self.run_button.grid(row=0, column=2, padx=10, pady=5)

        # Make the frames expand dynamically when the window is resized
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_rowconfigure(1, weight=1)
        self.root.grid_columnconfigure(0, weight=1)

    def run_code(self):
        # Use the run_code function imported from run_button.py
        run_code(self.editor_frame, self.output_frame)

    def dummy_action(self):
        return

def main():
    root = tk.Tk()
    app = CodeEditorApp(root)
    root.mainloop()

if __name__ == "__main__":
    main()
