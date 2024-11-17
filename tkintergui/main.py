# main.py
import tkinter as tk
from editor import CodeEditor
from output import OutputPanel
from run_button import RunButton

class CodeEditorApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Code Editor")

        # Set up the frames
        self.editor_frame = CodeEditor(self.root)
        self.editor_frame.grid(row=0, column=0, padx=10, pady=10)

        self.output_frame = OutputPanel(self.root)
        self.output_frame.grid(row=1, column=0, padx=10, pady=10)

        self.run_button = RunButton(self.root, self.editor_frame, self.output_frame)
        self.run_button.grid(row=2, column=0, padx=10, pady=10)

def main():
    root = tk.Tk()
    app = CodeEditorApp(root)
    root.mainloop()

if __name__ == "__main__":
    main()
