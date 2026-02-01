/// Get Current Time Tool
///
/// A simple tool that returns the current date and time.
/// This demonstrates the ToolExecutor pattern without any risky operations.

use super::ToolExecutor;
use crate::api::Tool;
use serde_json::{json, Value};

pub struct GetTimeTool;

impl GetTimeTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GetTimeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolExecutor for GetTimeTool {
    fn name(&self) -> &str {
        "get_current_time"
    }

    fn definition(&self) -> Tool {
        Tool::new(
            "get_current_time",
            "Get the current date and time. Use this when the user asks about the current time or date.",
            json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        )
    }

    fn execute(&self, _input: Value) -> Result<String, String> {
        // Get current time using std (no external crate needed)
        let now = std::time::SystemTime::now();
        let duration = now
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?;

        // Convert to readable format (basic, no chrono dependency)
        let secs = duration.as_secs();
        Ok(format!(
            "Current Unix timestamp: {} seconds since epoch",
            secs
        ))
    }
}
