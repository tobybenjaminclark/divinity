import os
from openai import OpenAI
import json

import openai

def divine_code(editor_frame, output_frame, parent):
    # Get code from the editor frame (textbox)
    code = editor_frame.get_code()
    if code.strip():
        output_frame.clear_output()
        make_code_divine(code, output_frame, editor_frame, parent)
    else:
        output_frame.show_output("Error: No code to run.")

def make_code_divine(code, output_frame, editor_frame, parent):
    """Send the code to ChatGPT and handle the response."""


    client = OpenAI(
    api_key = os.getenv("OPENAI_API_KEY"),
)
    
    threshold = 5 # max 5 tries
    counter = 0

    while counter < threshold :
        try:
                print(f"counter = {counter}")
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

                # Show the result in the output frame
                editor_frame.set_text(result)

                # call the type checker on the result
                
                correct_code = remove_comments(code)

                if parent.run_verifier(correct_code, output_frame) is True:
                     return
                
                counter +=1 
            
        except Exception as e:
                output_frame.show_output(f"Error: {str(e)}")
                counter +=1 

            

  
def remove_comments(code):
    """Remove comments wrapped in /* */."""
    # Regex to match anything between /* and */
    code_without_comments = re.sub(r'/\*.*?\*/', '', code, flags=re.DOTALL)
    return code_without_comments
