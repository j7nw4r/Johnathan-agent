/// Tool Registry - holds and manages available tools
///
/// When Claude requests a tool by name, we need to:
/// 1. Find the right tool
/// 2. Execute it
/// 3. Return the result
///
/// The registry provides this lookup capability.

use super::ToolExecutor;
use crate::api::Tool;
use std::collections::HashMap;

/// Holds all registered tools and provides lookup
pub struct ToolRegistry {
    /// Tools indexed by name for O(1) lookup
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool (takes ownership)
    pub fn register<T: ToolExecutor + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Box::new(tool));
    }

    /// Get tool definitions for sending to Claude
    pub fn definitions(&self) -> Vec<Tool> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Execute a tool by name with given input
    pub fn execute(&self, name: &str, input: serde_json::Value) -> Result<String, String> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(input),
            None => Err(format!("Unknown tool: {}", name)),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
