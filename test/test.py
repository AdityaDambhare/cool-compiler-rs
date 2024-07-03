import subprocess
import os

def run_coolc_on_examples(directory_path):
    
    files = [f for f in os.listdir(directory_path) if os.path.isfile(os.path.join(directory_path, f))]
    
   
    
    for file in files:
        try:
            file_path = os.path.join(directory_path, file)
        except:
            print(f"{file_path} is not a valid file")
            continue
        
        # Run coolc.exe on the file
        try:
            result = subprocess.run(['coolc.exe', file_path], capture_output=True, text=True)
        except:
            print(f"error reading {file_path}")
            continue
        # Capture and print the exit code
        if result.returncode == 0:
            print(f"{file_path} passed")
        else:
            print(f"{file_path} failed with exit code {result.returncode}" )
        
        # Optionally, print stdout and stderr
        # print(f"stdout: {result.stdout}")
        # print(f"stderr: {result.stderr}")

# Replace 'examples' with the path to your examples directory
run_coolc_on_examples('examples')