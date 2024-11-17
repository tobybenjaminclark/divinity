import tkinter as tk
from tkinter import DISABLED, scrolledtext


class OutputPanel(tk.Frame):
    def __init__(self, master):
        super().__init__(master)

        # Create a Text widget for displaying output
        self.output_text = scrolledtext.ScrolledText(self, wrap=tk.WORD, width=80, height=20, state=DISABLED)
        self.output_text.grid(row=0, column=0, padx=10, pady=10)

        # Create a Label widget to display "Right" or "Wrong"
        self.state_label = tk.Label(self, text="", width=20, bg="grey", fg="white", font=("Arial", 12))
        self.state_label.grid(row=0, column=1, padx=10, pady=10, sticky="n")

    def show_output(self, output):
        self.output_text.config(state=tk.NORMAL)
        self.output_text.delete("1.0", tk.END)
        self.output_text.insert(tk.END, output)
        self.output_text.config(state=tk.DISABLED)

    def append_output(self, output):
        self.output_text.config(state=tk.NORMAL)
        self.output_text.insert(tk.END, output + "\n")
        self.output_text.config(state=tk.DISABLED)

    def clear_output(self):
        self.output_text.config(state=tk.NORMAL)
        self.output_text.delete("1.0", tk.END)
        self.output_text.config(state=tk.DISABLED)

    def set_state(self, state):
        if state is True:
            self.state_label.config(text="✔️")
        else:
            self.state_label.config(text="❌")


# Define a custom class to redirect stdout
class StdoutRedirector:
    def __init__(self, output_panel):
        self.output_panel = output_panel

    def write(self, message):
        # Redirect the message to the OutputPanel
        self.output_panel.append_output(message)

    def flush(self):
        # Flush method is required for Python compatibility
        pass