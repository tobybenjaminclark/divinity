# output.py
import tkinter as tk

class OutputPanel(tk.Frame):
    def __init__(self, master):
        super().__init__(master)
        
        # Create a Text widget for displaying output
        self.output_text = tk.Text(self, wrap=tk.WORD, width=80, height=10, state=tk.DISABLED)
        self.output_text.grid(row=0, column=0)

    def show_output(self, output):
        self.output_text.config(state=tk.NORMAL)
        self.output_text.delete("1.0", tk.END)
        self.output_text.insert(tk.END, output)
        self.output_text.config(state=tk.DISABLED)

    def clear_output(self):
        self.output_text.config(state=tk.NORMAL)
        self.output_text.delete("1.0", tk.END)
        self.output_text.config(state=tk.DISABLED)
