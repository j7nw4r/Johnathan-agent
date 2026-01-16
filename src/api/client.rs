/// Claude API Client
///
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history, roles
/// Topic 6: Streaming Responses - SSE, real-time token display
/// Topic 8: Tool Use / Function Calling

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

// ============================================================================
// Tool Definitions
// ============================================================================

/// A tool that Claude can use
#[derive(Debug, Serialize, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

impl Tool {
    /// Create a new tool definition
    pub fn new(name: &str, description: &str, input_schema: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
        }
    }
}

/// A tool call requested by Claude
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: Value,
}

// ============================================================================
// Message Types (now with content blocks for tool use)
// ============================================================================

/// Content block - can be text, tool_use, or tool_result
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },

    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

/// A message in the conversation (supports both simple text and content blocks)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    #[serde(flatten)]
    pub content: MessageContent,
}

/// Message content - either a simple string or array of content blocks
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text { content: String },
    Blocks { content: Vec<ContentBlock> },
}

impl Message {
    /// Create a simple user text message
    pub fn user(text: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: MessageContent::Text {
                content: text.to_string(),
            },
        }
    }

    /// Create a simple assistant text message
    pub fn assistant(text: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: MessageContent::Text {
                content: text.to_string(),
            },
        }
    }

    /// Create an assistant message with tool use (for reconstructing history)
    pub fn assistant_tool_use(tool_calls: &[ToolCall]) -> Self {
        let blocks: Vec<ContentBlock> = tool_calls
            .iter()
            .map(|tc| ContentBlock::ToolUse {
                id: tc.id.clone(),
                name: tc.name.clone(),
                input: tc.input.clone(),
            })
            .collect();

        Self {
            role: "assistant".to_string(),
            content: MessageContent::Blocks { content: blocks },
        }
    }

    /// Create a user message with tool results
    pub fn tool_results(results: Vec<(String, String)>) -> Self {
        let blocks: Vec<ContentBlock> = results
            .into_iter()
            .map(|(tool_use_id, content)| ContentBlock::ToolResult {
                tool_use_id,
                content,
            })
            .collect();

        Self {
            role: "user".to_string(),
            content: MessageContent::Blocks { content: blocks },
        }
    }
}

// ============================================================================
// API Request/Response
// ============================================================================

#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,
}

/// Structured response from chat
#[derive(Debug)]
pub struct ChatResponse {
    pub text: String,
    pub stop_reason: String,
    pub tool_calls: Vec<ToolCall>,
}

impl ChatResponse {
    /// Check if the model wants to use tools
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }
}

// ============================================================================
// Streaming SSE Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct StreamContentBlockStart {
    #[serde(rename = "type")]
    event_type: String,
    index: usize,
    content_block: Option<StreamContentBlock>,
}

#[derive(Debug, Deserialize)]
struct StreamContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    id: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamContentBlockDelta {
    #[serde(rename = "type")]
    event_type: String,
    index: usize,
    delta: Option<StreamDelta>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    #[serde(rename = "type")]
    delta_type: String,
    text: Option<String>,
    partial_json: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamMessageDelta {
    #[serde(rename = "type")]
    event_type: String,
    delta: Option<StreamMessageDeltaData>,
}

#[derive(Debug, Deserialize)]
struct StreamMessageDeltaData {
    stop_reason: Option<String>,
}

// ============================================================================
// API Functions
// ============================================================================

/// Send messages with streaming and tool support
pub fn send_messages_streaming<F>(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
    tools: Vec<Tool>,
    mut on_text_chunk: F,
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
        tools,
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

    // Parse SSE stream
    let reader = BufReader::new(response);
    let mut full_text = String::new();
    let mut stop_reason = "unknown".to_string();
    let mut tool_calls: Vec<ToolCall> = Vec::new();

    // Track current tool being built (for streaming tool input)
    let mut current_tool_id: Option<String> = None;
    let mut current_tool_name: Option<String> = None;
    let mut current_tool_json = String::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Read error: {}", e))?;

        if let Some(data) = line.strip_prefix("data: ") {
            if data == "[DONE]" {
                continue;
            }

            // content_block_start - might be text or tool_use
            if let Ok(event) = serde_json::from_str::<StreamContentBlockStart>(data) {
                if event.event_type == "content_block_start" {
                    if let Some(block) = event.content_block {
                        if block.block_type == "tool_use" {
                            current_tool_id = block.id;
                            current_tool_name = block.name;
                            current_tool_json.clear();
                        }
                    }
                }
            }

            // content_block_delta - text or tool input JSON
            if let Ok(event) = serde_json::from_str::<StreamContentBlockDelta>(data) {
                if event.event_type == "content_block_delta" {
                    if let Some(delta) = event.delta {
                        // Text delta
                        if let Some(text) = delta.text {
                            on_text_chunk(&text);
                            full_text.push_str(&text);
                        }
                        // Tool input JSON delta
                        if let Some(json) = delta.partial_json {
                            current_tool_json.push_str(&json);
                        }
                    }
                }
            }

            // content_block_stop - finalize tool if we were building one
            if data.contains("\"type\":\"content_block_stop\"") {
                if let (Some(id), Some(name)) = (current_tool_id.take(), current_tool_name.take()) {
                    let input: Value = serde_json::from_str(&current_tool_json)
                        .unwrap_or(Value::Object(serde_json::Map::new()));
                    tool_calls.push(ToolCall { id, name, input });
                    current_tool_json.clear();
                }
            }

            // message_delta - stop_reason
            if let Ok(event) = serde_json::from_str::<StreamMessageDelta>(data) {
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
        tool_calls,
    })
}

/// Send messages without streaming
pub fn send_messages(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
    tools: Vec<Tool>,
) -> Result<ChatResponse, String> {
    send_messages_streaming(api_key, messages, system_prompt, tools, |_| {})
}
