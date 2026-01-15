/// Claude API Client
///
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history, roles
/// Topic 6: Streaming Responses - SSE, real-time token display

use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};

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

/// Request body for the Claude API (non-streaming)
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    stream: bool,
}

/// Delta content in streaming response
#[derive(Debug, Deserialize)]
struct TextDelta {
    #[serde(rename = "type")]
    delta_type: String,
    text: Option<String>,
}

/// Streaming event data for content_block_delta
#[derive(Debug, Deserialize)]
struct ContentBlockDelta {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<TextDelta>,
}

/// Streaming event data for message_delta (contains stop_reason)
#[derive(Debug, Deserialize)]
struct MessageDelta {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<MessageDeltaData>,
}

#[derive(Debug, Deserialize)]
struct MessageDeltaData {
    stop_reason: Option<String>,
}

/// Structured response from chat
#[derive(Debug)]
pub struct ChatResponse {
    pub text: String,
    pub stop_reason: String,
}

/// Send messages with streaming, calling a callback for each text chunk
///
/// This enables real-time display of tokens as they arrive.
/// The callback receives each text delta as it's streamed.
pub fn send_messages_streaming<F>(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
    mut on_chunk: F,
) -> Result<ChatResponse, String>
where
    F: FnMut(&str),
{
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 4096,
        messages,
        system: system_prompt.map(|s| s.to_string()),
        stream: true,
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

    // Read SSE stream line by line
    let reader = BufReader::new(response);
    let mut full_text = String::new();
    let mut stop_reason = "unknown".to_string();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Read error: {}", e))?;

        // SSE format: "data: {json}"
        if let Some(data) = line.strip_prefix("data: ") {
            // Skip [DONE] marker if present
            if data == "[DONE]" {
                continue;
            }

            // Try to parse as content_block_delta (contains text)
            if let Ok(event) = serde_json::from_str::<ContentBlockDelta>(data) {
                if event.event_type == "content_block_delta" {
                    if let Some(delta) = event.delta {
                        if let Some(text) = delta.text {
                            on_chunk(&text);
                            full_text.push_str(&text);
                        }
                    }
                }
            }

            // Try to parse as message_delta (contains stop_reason)
            if let Ok(event) = serde_json::from_str::<MessageDelta>(data) {
                if event.event_type == "message_delta" {
                    if let Some(delta) = event.delta {
                        if let Some(reason) = delta.stop_reason {
                            stop_reason = reason;
                        }
                    }
                }
            }
        }
    }

    Ok(ChatResponse {
        text: full_text,
        stop_reason,
    })
}

/// Send messages without streaming (waits for full response)
pub fn send_messages(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
) -> Result<ChatResponse, String> {
    // Use streaming internally but collect all chunks
    let mut text = String::new();
    let response = send_messages_streaming(
        api_key,
        messages,
        system_prompt,
        |chunk| text.push_str(chunk),
    )?;

    Ok(ChatResponse {
        text: response.text,
        stop_reason: response.stop_reason,
    })
}
