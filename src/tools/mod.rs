/// Tools module - executable capabilities for the agent
///
/// Topic 9: Designing a Tool System
///
/// Key concepts:
/// - ToolExecutor trait: uniform interface for all tools
/// - ToolRegistry: holds and looks up available tools
/// - Each tool: definition (for Claude) + execution (actual work)

mod get_time;
mod registry;

pub use get_time::GetTimeTool;
pub use registry::ToolRegistry;

use crate::api::Tool;
use serde_json::Value;

/// The core trait that all tools must implement
///
/// This is the heart of the tool system. Every tool:
/// 1. Has a name (for lookup when Claude requests it)
/// 2. Provides a definition (sent to Claude so it knows what's available)
/// 3. Can execute with JSON input and return a string result
pub trait ToolExecutor: Send + Sync {
    /// Unique name of the tool (must match what's sent to Claude)
    fn name(&self) -> &str;

    /// The tool definition to send to Claude's API
    fn definition(&self) -> Tool;

    /// Execute the tool with the given input
    /// Returns Ok(output) on success, Err(error_message) on failure
    fn execute(&self, input: Value) -> Result<String, String>;
}
