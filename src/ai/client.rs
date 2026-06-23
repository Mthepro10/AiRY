use serde::{Deserialize, Serialize};

const MODEL: &str = "google/gemma-4-31B-it:hf-inference";
const API_URL: &str = "https://api-inference.huggingface.co/v1/chat/completions";

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct Request {
    model: &'static str,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

fn load_token() -> Result<String, String> {
    let env = std::fs::read_to_string(".env")
        .map_err(|_| "Could not read .env file — make sure it exists with HF_TOKEN=...".to_string())?;

    for line in env.lines() {
        if let Some(value) = line.strip_prefix("HF_TOKEN=") {
            let token = value.trim().trim_matches('"').to_string();
            if token.is_empty() {
                return Err("HF_TOKEN is empty in .env".to_string());
            }
            return Ok(token);
        }
    }
    Err("HF_TOKEN not found in .env".to_string())
}

pub fn translate(prompt: String) -> Result<String, String> {
    let token = load_token()?;

    let body = Request {
        model: MODEL,
        messages: vec![Message { role: "user", content: prompt }],
    };

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(API_URL)
        .bearer_auth(&token)
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().unwrap_or_default();
        return Err(format!("API error {status}: {text}"));
    }

    let parsed: Response = response
        .json()
        .map_err(|e| format!("Failed to parse API response: {e}"))?;

    parsed
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or_else(|| "API returned empty response".to_string())
}