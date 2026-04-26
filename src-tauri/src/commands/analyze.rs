use std::fs;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const VENICE_ENDPOINT: &str = "https://api.venice.ai/api/v1/chat/completions";
const VENICE_MODEL: &str = "qwen3-vl-235b-a22b";
const VENICE_KEY_PATH: &str = "/home/tux/Downloads/JacksKeys/Venice.txt";
const VISION_PROMPT: &str = "Look at this screenshot carefully. If it contains a multiple choice question, identify the question and all answer options. Then determine the correct answer. Respond in JSON format: {\"question\": \"...\", \"options\": [\"A. ...\", \"B. ...\", \"C. ...\", \"D. ...\"], \"answer\": \"B\", \"explanation\": \"...\"}";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question: String,
    pub options: Vec<String>,
    pub answer: String,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: String,
}

#[tauri::command]
pub async fn analyze_screenshot(image_base64: String, api_key: String) -> Result<Answer, String> {
    let key = resolve_api_key(api_key)?;
    let client = Client::new();
    let request_body = json!({
        "model": VENICE_MODEL,
        "messages": [
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": VISION_PROMPT},
                    {
                        "type": "image_url",
                        "image_url": {"url": format!("data:image/png;base64,{image_base64}")}
                    }
                ]
            }
        ],
        "temperature": 0.0,
        "venice_parameters": {
            "strip_thinking_response": true
        }
    });

    let response = client
        .post(VENICE_ENDPOINT)
        .bearer_auth(key)
        .json(&request_body)
        .send()
        .await
        .map_err(|error| format!("Venice request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        return Err(format!("Venice API returned status {status}."));
    }

    let completion = response
        .json::<ChatCompletionResponse>()
        .await
        .map_err(|error| format!("Failed to parse Venice response: {error}"))?;
    let content = completion
        .choices
        .first()
        .map(|choice| choice.message.content.as_str())
        .ok_or_else(|| "Venice response did not include an answer.".to_string())?;

    parse_answer(content)
}

fn resolve_api_key(api_key: String) -> Result<String, String> {
    let key = if api_key.trim().is_empty() {
        fs::read_to_string(VENICE_KEY_PATH)
            .map_err(|error| format!("Failed to read Venice API key file: {error}"))?
    } else {
        api_key
    };

    let trimmed = key.trim().to_string();
    if trimmed.is_empty() {
        Err("Venice API key is empty.".to_string())
    } else {
        Ok(trimmed)
    }
}

fn parse_answer(content: &str) -> Result<Answer, String> {
    let cleaned = extract_json_object(content)
        .ok_or_else(|| "Venice response did not contain a JSON object.".to_string())?;
    serde_json::from_str::<Answer>(cleaned)
        .map_err(|error| format!("Failed to parse answer JSON: {error}"))
}

fn extract_json_object(content: &str) -> Option<&str> {
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    if start <= end {
        Some(&content[start..=end])
    } else {
        None
    }
}
