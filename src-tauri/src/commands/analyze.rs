use std::fs;
use std::time::Duration;

use reqwest::header::{ACCEPT, ACCEPT_ENCODING, CONTENT_ENCODING};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

const GEMINI_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";
const GEMINI_KEY_PATH: &str = "/home/tux/Downloads/JacksKeys/Google.txt";
const VISION_PROMPT: &str = "Analyze this screenshot. You MUST respond with valid JSON only, no other text. If it contains a multiple choice question, use this format: {\"question\": \"...\", \"options\": [\"A. ...\", \"B. ...\", \"C. ...\", \"D. ...\"], \"answer\": \"B\", \"explanation\": \"...\"}. If there is NO question, respond with: {\"question\": \"No question found\", \"options\": [], \"answer\": \"N/A\", \"explanation\": \"The screenshot does not contain a recognizable question.\"}. Do NOT include markdown, code fences, or any text outside the JSON.";

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
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .no_gzip()
        .build()
        .map_err(|error| format!("Failed to build Gemini HTTP client: {error}"))?;
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
        .header(ACCEPT, "application/json")
        .header(ACCEPT_ENCODING, "identity")
        .json(&request_body)
        .send()
        .await
        .map_err(|error| format!("Gemini request failed: {error}"))?;

    let status = response.status();
    let content_encoding = response
        .headers()
        .get(CONTENT_ENCODING)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("identity")
        .to_string();

    eprintln!("Gemini response status: {status}, content-encoding: {content_encoding}");

    let response_bytes = response
        .bytes()
        .await
        .map_err(|error| format!("Failed to read Gemini response body: {error}"))?;
    let response_text = String::from_utf8(response_bytes.to_vec()).map_err(|error| {
        format!("Gemini response was not UTF-8 (content-encoding: {content_encoding}): {error}")
    })?;
    eprintln!("Gemini response preview: {}", preview(&response_text, 200));

    if !status.is_success() {
        return Err(format!(
            "Gemini API returned status {status}. Body: {}",
            preview(&response_text, 200)
        ));
    }

    let completion: GeminiResponse = serde_json::from_str(&response_text)
        .map_err(|error| format!("Failed to parse Gemini response: {error}. Body: {}", preview(&response_text, 200)))?;
    let content = completion
        .candidates
        .first()
        .and_then(|candidate| candidate.content.parts.first())
        .map(|part| part.text.as_str())
        .ok_or_else(|| "Gemini response did not include an answer.".to_string())?;

    parse_answer(content).map_err(|error| {
        format!("{error} (raw response: {:?})", preview(content, 200))
    })
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
        .ok_or_else(|| format!("Gemini did not return JSON. Response: {}", preview(content, 200)))?;
    serde_json::from_str::<Answer>(cleaned)
        .map_err(|error| format!("Failed to parse answer JSON: {error}"))
}

fn preview(content: &str, max_chars: usize) -> String {
    content.chars().take(max_chars).collect()
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
