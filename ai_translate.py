import sys
import os
import urllib.request
import json

def load_token():
    try:
        for line in open(".env", encoding="utf-8"):
            if line.startswith("HACKCLUB_API_KEY="):
                return line.strip().split("=", 1)[1]
    except FileNotFoundError:
        pass
    return None

API_KEY = load_token()

if not API_KEY:
    print("ERROR: HACKCLUB_API_KEY missing in .env", file=sys.stderr)
    sys.exit(1)

try:
    prompt = open("_prompt_tmp.txt", "r", encoding="utf-8").read()
except FileNotFoundError:
    print("ERROR: _prompt_tmp.txt not found", file=sys.stderr)
    sys.exit(1)

try:
    data = json.dumps({
        "model": "tencent/hy3:free",
        "messages": [{"role": "user", "content": prompt}],
        "stream": False
    }).encode("utf-8")

    req = urllib.request.Request(
        "https://ai.hackclub.com/proxy/v1/chat/completions",
        data=data,
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {API_KEY}"
        },
        method="POST"
    )

    with urllib.request.urlopen(req) as response:
        result = json.loads(response.read().decode("utf-8"))
        print(result["choices"][0]["message"]["content"])

except Exception as e:
    print(f"ERROR: {e}", file=sys.stderr)
    sys.exit(1)