/// Claude API Client
///
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history, roles

use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

/// A message in the conversation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

/// Request body for the Claude API
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

/// A content block in the response
#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

/// Response from the Claude API
#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
}

/// Structured response from chat
#[derive(Debug)]
pub struct ChatResponse {
    pub text: String,
    pub stop_reason: String,
}

/// Send messages to Claude with optional system prompt
///
/// This is the full-featured API call:
/// - Accepts conversation history (Vec<Message>)
/// - Supports system prompts for agent persona
/// - Returns structured response with stop_reason
pub fn send_messages(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
) -> Result<ChatResponse, String> {
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 4096,
        messages,
        system: system_prompt.map(|s| s.to_string()),
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let api_response: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let text = api_response
        .content
        .iter()
        .filter_map(|block| block.text.clone())
        .collect::<Vec<_>>()
        .join("");

    Ok(ChatResponse {
        text,
        stop_reason: api_response.stop_reason.unwrap_or_else(|| "unknown".to_string()),
    })
}

/// Convenience function for single message (no history)
pub fn send_message(api_key: &str, user_message: &str) -> Result<String, String> {
    let response = send_messages(
        api_key,
        vec![Message::user(user_message)],
        None,
    )?;
    Ok(response.text)
}
