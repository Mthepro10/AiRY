from huggingface_hub import InferenceClient
from config import MODEL_NAME, HF_TOKEN

client = InferenceClient(model=MODEL_NAME, token=HF_TOKEN)

def call_gemma(prompt: str) -> str:
    response = client.chat_completion(
        messages=[{"role": "user", "content": prompt}]
    )
    return response.choices[0].message.content