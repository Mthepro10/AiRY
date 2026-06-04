import os
from dotenv import load_dotenv

load_dotenv()

HF_TOKEN = os.getenv("HF_TOKEN")
MODEL_NAME = "google/gemma-4-31B-it"

if not HF_TOKEN:
    raise Exception("HF_TOKEN missing in .env")