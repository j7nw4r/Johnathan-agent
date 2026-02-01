# Topic 9: Designing a Tool System

## 1. Present

In Topic 8, we built the API plumbing - the ability to send tool definitions and parse tool calls. Now we need the **internal architecture** to:

1. Define tools in a uniform way
2. Look up tools by name when Claude requests them
3. Execute tools and return results

This is a classic abstraction problem: different tools do different things, but they share a common interface.

---

## 2. Relate

Right now we have:
- `Tool` struct (definition sent to Claude)
- `ToolCall` struct (what Claude requests)
- `ChatResponse.tool_calls` (parsed from the stream)

But there's a gap: when Claude says "use tool X with input Y", what code actually runs?

---

## 3. Explain

### The Pattern: Trait + Registry

Every tool, regardless of what it does, follows the same pattern:

```
JSON Input → Execute → String Output
```

A file reader takes a path, returns contents. A bash runner takes a command, returns output. This shared behavior is captured in a trait:

```rust
pub trait ToolExecutor: Send + Sync {
    fn name(&self) -> &str;
    fn definition(&self) -> Tool;
    fn execute(&self, input: Value) -> Result<String, String>;
}
```

Why these three methods?

| Method | Purpose |
|--------|---------|
| `name()` | Lookup key when Claude says "use tool X" |
| `definition()` | What we send to Claude (name, description, schema) |
| `execute()` | The actual work - parse input, do the thing, return result |

### The Registry

We need a container that holds tools and provides lookup:

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn register<T: ToolExecutor + 'static>(&mut self, tool: T);
    pub fn definitions(&self) -> Vec<Tool>;
    pub fn execute(&self, name: &str, input: Value) -> Result<String, String>;
}
```

The registry:
- Stores tools in a HashMap for O(1) lookup by name
- Collects all definitions for sending to Claude
- Routes execution requests to the right tool

### Why This Design?

**1. Uniform Interface**

All tools look the same to the agent loop. Adding a new tool doesn't require changing any calling code.

**2. Easy to Extend**

Adding a new tool:
1. Create a struct implementing `ToolExecutor`
2. Register it in main.rs

No changes to API code, REPL code, or other tools.

**3. Separation of Concerns**

- API layer doesn't know how tools work internally
- Tool implementations don't know about HTTP or streaming
- Registry just maps names to implementations

---

## 4. Implement

**src/tools/mod.rs:**
```rust
use crate::api::Tool;
use serde_json::Value;

pub trait ToolExecutor: Send + Sync {
    fn name(&self) -> &str;
    fn definition(&self) -> Tool;
    fn execute(&self, input: Value) -> Result<String, String>;
}
```

**src/tools/registry.rs:**
```rust
use super::ToolExecutor;
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    pub fn register<T: ToolExecutor + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tools.insert(name, Box::new(tool));
    }

    pub fn definitions(&self) -> Vec<Tool> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    pub fn execute(&self, name: &str, input: Value) -> Result<String, String> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(input),
            None => Err(format!("Unknown tool: {}", name)),
        }
    }
}
```

**src/tools/get_time.rs (example tool):**
```rust
use super::ToolExecutor;
use crate::api::Tool;
use serde_json::{json, Value};

pub struct GetTimeTool;

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
        let duration = now.duration_since(std::time::UNIX_EPOCH)?;
        Ok(format!("Current Unix timestamp: {}", duration.as_secs()))
    }
}
```

**main.rs - wiring it up:**
```rust
use tools::{GetTimeTool, ToolRegistry};

fn main() {
    // ... setup ...

    let mut registry = ToolRegistry::new();
    registry.register(GetTimeTool::new());

    // Pass registry to REPL
    run_repl(&api_key, &registry, verbose);
}

fn eval_streaming(..., registry: &ToolRegistry, ...) {
    let tools = registry.definitions();
    // Send tools with API request
}
```

---

## 5. Review

**The architecture:**

```
main.rs
├── ToolRegistry::new()
├── registry.register(GetTimeTool)
└── run_repl(registry)
    └── eval_streaming(registry)
        └── registry.definitions() → sent to Claude

tools/
├── mod.rs         # ToolExecutor trait
├── registry.rs    # ToolRegistry
└── get_time.rs    # Example implementation
```

**What's connected:**

| Component | Status |
|-----------|--------|
| Tool definitions | ✅ Sent to Claude |
| Tool lookup | ✅ Registry.execute() |
| Execution | ✅ GetTimeTool.execute() |
| Loop integration | ❌ Not yet (Topic 11) |

**What's still missing:**

The tool system is **defined** but not **connected**. When Claude says "use get_current_time", we don't actually:
1. Check `response.has_tool_calls()`
2. Call `registry.execute()`
3. Send results back
4. Continue the loop

That's Topic 11.

---

## Key Takeaways

- Trait = uniform interface for all tools
- Registry = HashMap-based lookup by name
- Three methods: `name()`, `definition()`, `execute()`
- Separation of concerns: tools don't know about HTTP, API doesn't know tool internals
- Next: implement real tools (Topic 10) and wire up the loop (Topic 11)
