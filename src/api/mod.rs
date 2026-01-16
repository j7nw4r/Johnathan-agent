/// API module - handles communication with Claude API
///
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history
/// Topic 6: Streaming Responses
/// Topic 8: Tool Use / Function Calling

mod client;

pub use client::{
    send_messages, send_messages_streaming, ChatResponse, Message, Tool, ToolCall,
};
