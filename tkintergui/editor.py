# editor.py
import tkinter as tk
from tkinter import scrolledtext

class CodeEditor(tk.Frame):
    def __init__(self, master):
        super().__init__(master)
        
        # Create a ScrolledText widget for code input
        self.editor = scrolledtext.ScrolledText(self, wrap=tk.WORD, width=80, height=20)
        self.editor.grid(row=0, column=0)

    def get_code(self):
        return self.editor.get("1.0", tk.END)

    def clear_code(self):
        self.editor.delete("1.0", tk.END)

    def set_text(self, text):
        """Sets the content of the editor to the provided text."""
        self.editor.delete("1.0", tk.END)  # Clear the current content
        self.editor.insert(tk.END, text)    # Insert the new text
