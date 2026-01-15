/// Claude API Client
///
/// This module handles HTTP communication with the Anthropic API.
/// Key concepts:
/// - HTTP POST request with JSON body
/// - Authorization headers
/// - Request/Response serialization

use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

/// A message in the conversation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Request body for the Claude API
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
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

/// Send a message to Claude and get a response
///
/// This is the core HTTP interaction:
/// 1. Build the request with headers and JSON body
/// 2. Send it to the API endpoint
/// 3. Parse the JSON response
/// 4. Extract the text content
pub fn send_message(api_key: &str, user_message: &str) -> Result<String, String> {
    // Build the request body
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        }],
    };

    // Create HTTP client and send request
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // Check for HTTP errors
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    // Parse response JSON
    let api_response: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Extract text from content blocks
    let text = api_response
        .content
        .iter()
        .filter_map(|block| block.text.clone())
        .collect::<Vec<_>>()
        .join("");

    Ok(text)
}
