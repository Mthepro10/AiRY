from file_loader import read_airy_file
from llama_client import call_llama
from prompt import build_prompt

def main():
    source_code = read_airy_file()

    prompt = build_prompt(source_code)
    result = call_llama(prompt)


    with open("ai_compile.txt", "w", encoding="utf-8") as f:
        f.write(result)


if __name__ == "__main__":
    main()