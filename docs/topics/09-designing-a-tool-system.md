# Topic 9: Designing a Tool System

## The Concept

In Topic 8, we built the API plumbing to send tool definitions and parse tool calls. Now we need the **internal architecture** to:

1. Define tools in a uniform way
2. Look up tools by name when Claude requests them
3. Execute tools and return results

This is a classic abstraction problem: different tools do different things, but they share a common interface.

## The Design

### The ToolExecutor Trait

Every tool, regardless of what it does, follows the same pattern:

```
JSON Input → Execute → String Output
```

A file reader takes a path, returns contents. A bash runner takes a command, returns output. This shared behavior is captured in a trait:

```rust
// src/tools/mod.rs

pub trait ToolExecutor: Send + Sync {
    /// Unique name of the tool (must match what's sent to Claude)
    fn name(&self) -> &str;

    /// The tool definition to send to Claude's API
    fn definition(&self) -> Tool;

    /// Execute the tool with the given input
    fn execute(&self, input: Value) -> Result<String, String>;
}
```

Why these three methods?

| Method | Purpose |
|--------|---------|
| `name()` | Lookup key when Claude says "use tool X" |
| `definition()` | What we send to Claude (name, description, schema) |
| `execute()` | The actual work - parse input, do the thing, return result |

### The ToolRegistry

We need a container that holds tools and provides lookup:

```rust
// src/tools/registry.rs

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
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
    pub fn execute(&self, name: &str, input: Value) -> Result<String, String> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(input),
            None => Err(format!("Unknown tool: {}", name)),
        }
    }
}
```

### Example Tool: GetTimeTool

A simple tool to demonstrate the pattern:

```rust
// src/tools/get_time.rs

use super::ToolExecutor;
use crate::api::Tool;
use serde_json::{json, Value};

pub struct GetTimeTool;

impl GetTimeTool {
    pub fn new() -> Self {
        Self
    }
}

impl ToolExecutor for GetTimeTool {
    fn name(&self) -> &str {
        "get_current_time"
    }

    fn definition(&self) -> Tool {
        Tool::new(
            "get_current_time",
            "Get the current date and time",
            json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        )
    }

    fn execute(&self, _input: Value) -> Result<String, String> {
        let now = std::time::SystemTime::now();
        let duration = now
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?;

        Ok(format!("Current Unix timestamp: {} seconds", duration.as_secs()))
    }
}
```

## Wiring It Up

### Module Structure

```
src/
  tools/
    mod.rs       # ToolExecutor trait, re-exports
    registry.rs  # ToolRegistry
    get_time.rs  # GetTimeTool
```

```rust
// src/tools/mod.rs
mod get_time;
mod registry;

pub use get_time::GetTimeTool;
pub use registry::ToolRegistry;

// ... trait definition ...
```

### Setup in main.rs

```rust
use tools::{GetTimeTool, ToolRegistry};

fn main() {
    // ... API key setup ...

    // Create and populate registry
    let mut registry = ToolRegistry::new();
    registry.register(GetTimeTool::new());

    if verbose {
        println!("[tools registered: {}]", registry.definitions().len());
    }

    // Pass registry to REPL
    run_repl(&api_key, &registry, verbose);
}
```

### Sending Tool Definitions

```rust
fn eval_streaming(
    messages: Vec<Message>,
    api_key: &str,
    registry: &ToolRegistry,
    verbose: bool,
) -> String {
    // Get definitions from registry
    let tools = registry.definitions();

    let result = api::send_messages_streaming(
        api_key,
        messages,
        Some(SYSTEM_PROMPT),
        tools,  // Sent with every request
        |chunk| { /* ... */ },
    );

    // ...
}
```

## Why This Design?

### 1. Uniform Interface

All tools look the same to the agent loop. Adding a new tool doesn't require changing any calling code.

### 2. Easy to Extend

Adding a new tool:
1. Create a new file with a struct implementing `ToolExecutor`
2. Add it to mod.rs
3. Register it in main.rs

No changes to API code, REPL code, or other tools.

### 3. Separation of Concerns

- **API layer** doesn't know how tools work internally
- **Tool implementations** don't know about HTTP or streaming
- **Registry** just maps names to implementations

### 4. Testability

Each tool can be tested independently:

```rust
#[test]
fn test_get_time() {
    let tool = GetTimeTool::new();
    let result = tool.execute(json!({}));
    assert!(result.is_ok());
    assert!(result.unwrap().contains("timestamp"));
}
```

## What's Still Missing

The tool system is **defined** but not **connected**. When Claude says "use get_current_time", we:

- ✅ Have the tool registered
- ✅ Can look it up by name
- ✅ Can execute it
- ❌ Don't actually do any of this yet!

The pieces exist, but the loop in `eval_streaming` doesn't check for tool calls or execute them.

## Key Takeaways

1. **Trait = interface** - `ToolExecutor` defines what all tools must do
2. **Registry = lookup** - HashMap for O(1) tool access by name
3. **Separation** - Each tool is self-contained, easy to add/modify
4. **Three methods** - `name()`, `definition()`, `execute()`
5. **Not yet wired** - Topic 11 completes the loop

## What's Next

Topic 10 implements real, useful tools (read_file, bash). Topic 11 wires everything together into a complete tool use loop.
