# Topic 8: What is Tool Use / Function Calling?

## The Concept

Tool use (also called function calling) is what transforms a chatbot into an agent. Instead of just generating text, the LLM can request to **use tools** that interact with the real world.

```
Without tools:         With tools:
User: "Read foo.txt"   User: "Read foo.txt"
LLM: "I can't access   LLM: [tool_use: read_file("foo.txt")]
      your files..."   Agent: [executes, returns contents]
                       LLM: "The file contains..."
```

## How It Works

### 1. Define Available Tools

We tell Claude what tools exist and how to use them:

```json
{
  "tools": [
    {
      "name": "read_file",
      "description": "Read the contents of a file",
      "input_schema": {
        "type": "object",
        "properties": {
          "path": {
            "type": "string",
            "description": "Path to the file"
          }
        },
        "required": ["path"]
      }
    }
  ]
}
```

### 2. Claude Decides to Use a Tool

When Claude determines a tool would help, it responds with a `tool_use` block:

```json
{
  "content": [
    {
      "type": "tool_use",
      "id": "toolu_01ABC123",
      "name": "read_file",
      "input": {"path": "foo.txt"}
    }
  ],
  "stop_reason": "tool_use"
}
```

Note: `stop_reason` is `"tool_use"`, not `"end_turn"`.

### 3. Agent Executes and Returns Result

We execute the tool and send the result back:

```json
{
  "role": "user",
  "content": [
    {
      "type": "tool_result",
      "tool_use_id": "toolu_01ABC123",
      "content": "Contents of foo.txt:\nHello, world!"
    }
  ]
}
```

### 4. Claude Continues

Claude receives the result and can now respond (or use more tools):

```json
{
  "content": [
    {
      "type": "text",
      "text": "The file foo.txt contains a simple greeting: 'Hello, world!'"
    }
  ],
  "stop_reason": "end_turn"
}
```

## Code Implementation

### Tool Definition Structure

```rust
// src/api/client.rs

#[derive(Debug, Serialize, Clone)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,  // JSON Schema
}

impl Tool {
    pub fn new(name: &str, description: &str, input_schema: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
        }
    }
}
```

### Representing Tool Calls

```rust
/// A tool call requested by Claude
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,      // Unique ID to match with result
    pub name: String,    // Which tool to use
    pub input: Value,    // Arguments as JSON
}
```

### Content Blocks (Text and Tool Use)

Messages can contain multiple content blocks:

```rust
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
```

### Updated Message Type

Messages now support both simple text and content blocks:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    #[serde(flatten)]
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text { content: String },
    Blocks { content: Vec<ContentBlock> },
}
```

### Constructing Tool Result Messages

```rust
impl Message {
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
```

### Parsing Tool Use from Stream

Tool inputs arrive as streamed JSON:

```rust
// Track tool being built
let mut current_tool_id: Option<String> = None;
let mut current_tool_name: Option<String> = None;
let mut current_tool_json = String::new();

// On content_block_start with type "tool_use"
if block.block_type == "tool_use" {
    current_tool_id = block.id;
    current_tool_name = block.name;
    current_tool_json.clear();
}

// On content_block_delta with partial_json
if let Some(json) = delta.partial_json {
    current_tool_json.push_str(&json);
}

// On content_block_stop - finalize the tool call
if let (Some(id), Some(name)) = (current_tool_id.take(), current_tool_name.take()) {
    let input: Value = serde_json::from_str(&current_tool_json)?;
    tool_calls.push(ToolCall { id, name, input });
}
```

### Updated Response Structure

```rust
pub struct ChatResponse {
    pub text: String,              // Any text content
    pub stop_reason: String,       // "end_turn" or "tool_use"
    pub tool_calls: Vec<ToolCall>, // Requested tool uses
}

impl ChatResponse {
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }
}
```

## The Tool Use Loop

This creates a new loop pattern:

```
┌─────────────────────────────────────────────┐
│                                             │
│   User Input                                │
│       │                                     │
│       ▼                                     │
│   Send to Claude (with tools)               │
│       │                                     │
│       ▼                                     │
│   ┌─────────────────┐                       │
│   │ Check stop_reason│                      │
│   └────────┬────────┘                       │
│            │                                │
│    ┌───────┴───────┐                        │
│    │               │                        │
│    ▼               ▼                        │
│ "end_turn"    "tool_use"                    │
│    │               │                        │
│    ▼               ▼                        │
│  Done         Execute tools                 │
│                    │                        │
│                    ▼                        │
│              Send results ───────────┐      │
│                                      │      │
│                    ┌─────────────────┘      │
│                    │                        │
│                    ▼                        │
│              Back to Claude ────────────────┘
│                                             │
└─────────────────────────────────────────────┘
```

## Key Takeaways

1. **Tools = actions** - Transform LLM from advisor to actor
2. **JSON Schema** - Tools defined with structured input schemas
3. **Tool IDs** - Match tool_use to tool_result
4. **stop_reason** - Tells us if Claude wants to use tools or is done
5. **Loop required** - Tool use often requires multiple round-trips

## What's Next

Topic 9 designs the tool system—the `ToolExecutor` trait and `ToolRegistry` that let us define and manage tools in our codebase.
