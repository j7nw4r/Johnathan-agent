# Topic 8: What is Tool Use / Function Calling?

## 1. Present

Up until now, our agent can only **talk**. It receives input, thinks, and responds with text. But real agents can **act** - read files, run commands, search the web.

**Tool use** (also called function calling) is the mechanism that enables this:
1. You tell the model what tools exist
2. The model can request to use a tool instead of (or before) responding
3. Your agent executes the tool
4. You send the result back to the model
5. The model continues (maybe using more tools, or giving a final answer)

This is what separates a chatbot from an agent.

---

## 2. Relate

Remember our original agent loop from Topic 1?

```
while goal_not_achieved:
    observe()   ← User input OR tool result
    think()     ← LLM decides: respond OR use tool
    act()       ← Execute the tool
```

Tool use is what makes `act()` real. The LLM's response isn't just text - it can be a **tool request**.

---

## 3. Explain

### The Tool Use Flow

```
┌────────────────────────────────────────────────────────────────┐
│                      TOOL USE LOOP                             │
│                                                                │
│  User: "What's in main.rs?"                                    │
│           │                                                    │
│           ▼                                                    │
│  ┌─────────────────┐                                           │
│  │ Agent sends to  │  messages: [user: "What's in main.rs?"]   │
│  │ Claude API with │  tools: [read_file, write_file, ...]      │
│  │ tool definitions│                                           │
│  └────────┬────────┘                                           │
│           ▼                                                    │
│  ┌─────────────────┐                                           │
│  │ Claude responds │  "I'll read that file for you"            │
│  │ with tool_use   │  tool_use: {name: "read_file",            │
│  │                 │            input: {path: "main.rs"}}      │
│  └────────┬────────┘                                           │
│           ▼                                                    │
│  ┌─────────────────┐                                           │
│  │ Agent executes  │  → Actually reads main.rs                 │
│  │ the tool        │  → Gets file contents                     │
│  └────────┬────────┘                                           │
│           ▼                                                    │
│  ┌─────────────────┐                                           │
│  │ Agent sends     │  messages: [..., tool_result: contents]   │
│  │ result back     │                                           │
│  └────────┬────────┘                                           │
│           ▼                                                    │
│  ┌─────────────────┐                                           │
│  │ Claude continues│  "The file contains a main function..."   │
│  │ with final      │                                           │
│  │ response        │                                           │
│  └─────────────────┘                                           │
└────────────────────────────────────────────────────────────────┘
```

### Tool Definition Schema

Tools are defined with JSON Schema:

```json
{
  "name": "read_file",
  "description": "Read the contents of a file at the given path",
  "input_schema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "The path to the file to read"
      }
    },
    "required": ["path"]
  }
}
```

Key parts:
- **name**: Identifier the model uses to call it
- **description**: Helps the model know *when* to use it
- **input_schema**: What parameters it accepts

### The Response Changes

Without tools:
```json
{"type": "text", "text": "Hello!"}
```

With tools:
```json
{"type": "tool_use", "id": "tool_123", "name": "read_file", "input": {"path": "main.rs"}}
```

The `stop_reason` tells you what happened:
- `end_turn` - Model finished with text
- `tool_use` - Model wants to use a tool

### Tool Results

After executing, send the result back:

```json
{
  "role": "user",
  "content": [{
    "type": "tool_result",
    "tool_use_id": "tool_123",
    "content": "fn main() { ... }"
  }]
}
```

---

## 4. Implement

**Tool struct:**
```rust
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,  // JSON Schema
}
```

**ToolCall (parsed from response):**
```rust
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: Value,
}
```

**Content blocks for messages:**
```rust
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String },
}
```

**ChatResponse now includes tool_calls:**
```rust
pub struct ChatResponse {
    pub text: String,
    pub stop_reason: String,
    pub tool_calls: Vec<ToolCall>,
}
```

**Message helpers:**
```rust
// For history: record that assistant used tools
Message::assistant_tool_use(&tool_calls)

// For results: send tool outputs back
Message::tool_results(vec![
    (tool_id, result_string),
])
```

---

## 5. Review

**The key structures:**

| Type | Purpose |
|------|---------|
| `Tool` | Definition sent to API |
| `ToolCall` | Parsed request from Claude |
| `ContentBlock::ToolResult` | How you send results back |
| `stop_reason: "tool_use"` | Signals tool execution needed |

**The flow:**
```
1. Define tools: Vec<Tool>
2. Send with messages to API
3. If response.has_tool_calls():
   a. Execute each tool
   b. Build tool_results message
   c. Add to history
   d. Call API again
4. When stop_reason == "end_turn", done
```

---

## Key Takeaways

- Tools transform an LLM from advisor to actor
- Tool definitions use JSON Schema for structured input
- The agent loop becomes: ask → (tool call → execute → return result)* → final answer
- `stop_reason` tells you whether to execute tools or display response
