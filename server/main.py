import subprocess
import json
from flask import Flask, request

# Cloud Functions require a WSGI app
app = Flask(__name__)

@app.route("/", methods=["POST"])
def run_compiler():
    try:
        # Parse the incoming JSON request
        data = request.get_json()
        if not data or "code" not in data:
            return {"error": "Invalid input: 'code' field is required"}, 400
        
        # Write the code to a temporary file
        with open("input.txt", "w") as f:
            f.write(data["code"])
        
        # Call the Linux executable
        process = subprocess.run(
            ["./folder_main", "input.txt"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        # Capture the output
        result = {
            "stdout": process.stdout,
            "stderr": process.stderr,
            "return_code": process.returncode
        }

        # Return the result as JSON
        return json.dumps(result), 200

    except Exception as e:
        return {"error": str(e)}, 500

# Cloud Function entry point
def main(request):
    return app(request)
