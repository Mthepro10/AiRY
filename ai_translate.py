import os
import sys
from dotenv import load_dotenv
from huggingface_hub import InferenceClient

load_dotenv()

HF_TOKEN = os.getenv("HF_TOKEN")
MODEL_NAME = "google/gemma-4-31B-it"

if not HF_TOKEN:
    print("ERROR: HF_TOKEN missing in .env", file=sys.stderr)
    sys.exit(1)

try:
    prompt = open("_prompt_tmp.txt", "r", encoding="utf-8").read()
except FileNotFoundError:
    print("ERROR: _prompt_tmp.txt not found", file=sys.stderr)
    sys.exit(1)

try:
    client = InferenceClient(model=MODEL_NAME, token=HF_TOKEN)
    response = client.chat_completion(
        messages=[{"role": "user", "content": prompt}]
    )
    print(response.choices[0].message.content)
except Exception as e:
    print(f"ERROR: {e}", file=sys.stderr)
    sys.exit(1)