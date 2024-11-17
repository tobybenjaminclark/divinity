import os
from openai import OpenAI
import json

import openai

def divine_code(editor_frame, output_frame):
    # Get code from the editor frame (textbox)
    code = editor_frame.get_code()
    if code.strip():
        output_frame.clear_output()
        make_code_divine(code, output_frame, editor_frame)
    else:
        output_frame.show_output("Error: No code to run.")

def make_code_divine(code, output_frame, editor_frame):
    """Send the code to ChatGPT and handle the response."""
    
    # Prepare your prompt with the code to send to ChatGPT
    prompt = f"""
    The following code has certain "holes" (areas that need completion or fixing). Please fill in those holes and give back the full code.

    Code:
    {code}

    Please replace only the holes in the code. Do not make other changes.
    """

    client = OpenAI(
    api_key = os.getenv("OPENAI_API_KEY"),
)

    try:

        # Read the prompt template from prompt.txt
        with open("prompt.txt", "r") as prompt_file:
            prompt_template = prompt_file.read()

        # Insert the program into the prompt
        prompt = prompt_template + code


        completion = client.chat.completions.create(
            model="gpt-4",
            messages=[{"role": "assistant", "content": prompt}],
            temperature = 0,
        )
        
        # Extract the response content from the API
        result = completion.choices[0].message.content

        print(f"result: {result}")

        # Show the result in the output frame
        editor_frame.set_text(result)
    
    except Exception as e:
        output_frame.show_output(f"Error: {str(e)}")
