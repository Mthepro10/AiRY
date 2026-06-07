from file_loader import read_airy_file
from gemma_client import call_gemma
from prompt import build_prompt
import subprocess

def main():
    source_code = read_airy_file()

    prompt = build_prompt(source_code)
    result = call_gemma(prompt)


    with open("ai_compile.txt", "w", encoding="utf-8") as f:
        f.write(result)

    command = ["cargo", "run", "--release", "--", "interpreter/main.rs"]
    process = subprocess.run(command, capture_output=True, text=True)

    if process.returncode == 0 :
        print(process.stdout)
    else :
        print(process.stderr)


if __name__ == "__main__":
    main()