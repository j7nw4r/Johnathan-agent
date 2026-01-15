/// API module - handles communication with Claude API
///
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history

mod client;

pub use client::{send_message, send_messages, ChatResponse, Message};
