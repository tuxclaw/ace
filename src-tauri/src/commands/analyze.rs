use std::fs;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const GEMINI_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";
const GEMINI_KEY_PATH: &str = "/home/tux/Downloads/JacksKeys/Google.txt";
const VISION_PROMPT: &str = "Look at this screenshot carefully. If it contains a multiple choice question, identify the question and all answer options. Then determine the correct answer. Respond in JSON format: {\"question\": \"...\", \"options\": [\"A. ...\", \"B. ...\", \"C. ...\", \"D. ...\"], \"answer\": \"B\", \"explanation\": \"...\"}";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question: String,
    pub options: Vec<String>,
    pub answer: String,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Deserialize)]
struct GeminiPart {
    text: String,
}

#[tauri::command]
pub async fn analyze_screenshot(image_base64: String, api_key: String) -> Result<Answer, String> {
    let key = resolve_api_key(api_key)?;
    let client = Client::new();
    let request_body = json!({
        "contents": [
            {
                "parts": [
                    {"text": VISION_PROMPT},
                    {
                        "inlineData": {
                            "mimeType": "image/png",
                            "data": image_base64,
                        }
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.1,
            "maxOutputTokens": 2048,
        }
    });

    let response = client
        .post(GEMINI_ENDPOINT)
        .header("x-goog-api-key", key.as_str())
        .json(&request_body)
        .send()
        .await
        .map_err(|error| format!("Gemini request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        return Err(format!("Gemini API returned status {status}."));
    }

    let completion = response
        .json::<GeminiResponse>()
        .await
        .map_err(|error| format!("Failed to parse Gemini response: {error}"))?;
    let content = completion
        .candidates
        .first()
        .and_then(|candidate| candidate.content.parts.first())
        .map(|part| part.text.as_str())
        .ok_or_else(|| "Gemini response did not include an answer.".to_string())?;

    parse_answer(content)
}

fn resolve_api_key(api_key: String) -> Result<String, String> {
    let key = if api_key.trim().is_empty() {
        fs::read_to_string(GEMINI_KEY_PATH)
            .map_err(|error| format!("Failed to read Gemini API key file: {error}"))?
    } else {
        api_key
    };

    let trimmed = key.trim().to_string();
    if trimmed.is_empty() {
        Err("Gemini API key is empty.".to_string())
    } else {
        Ok(trimmed)
    }
}

fn parse_answer(content: &str) -> Result<Answer, String> {
    let cleaned = extract_json_object(content)
        .ok_or_else(|| "Gemini response did not contain a JSON object.".to_string())?;
    serde_json::from_str::<Answer>(cleaned)
        .map_err(|error| format!("Failed to parse answer JSON: {error}"))
}

fn extract_json_object(content: &str) -> Option<&str> {
    // Strip markdown code fences if present (```json ... ```)
    let stripped = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let start = stripped.find('{')?;
    let end = stripped.rfind('}')?;
    if start <= end {
        Some(&stripped[start..=end])
    } else {
        None
    }
}
