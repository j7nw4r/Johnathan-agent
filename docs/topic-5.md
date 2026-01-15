# Topic 5: The Anthropic API

## 1. Present

Topic 4 covered the HTTP mechanics. Now we go deeper into **what** we're sending - the structure of the Anthropic Messages API.

Key concepts:
- **System prompts** - Instructions that shape the assistant's behavior
- **Message history** - Multi-turn conversations, not just single exchanges
- **Roles** - User, assistant, and how they alternate
- **Stop reasons** - Why did the model stop generating?

This is where we give our agent its personality and memory.

---

## 2. Relate

The initial implementation sent single messages:
```rust
messages: vec![Message {
    role: "user".to_string(),
    content: user_message.to_string(),
}]
```

No system prompt. No history. Every message is a fresh start - the agent has amnesia.

We need to:
1. Add system prompt support
2. Maintain conversation history across turns
3. Handle the assistant's responses properly

---

## 3. Explain

### The Messages API Structure

```
┌─────────────────────────────────────────────────────────┐
│                 API Request Structure                   │
│                                                         │
│  {                                                      │
│    "model": "claude-sonnet-4-20250514",                │
│    "max_tokens": 1024,                                  │
│    "system": "You are a helpful assistant...",  ←─┐    │
│    "messages": [                                    │    │
│      {"role": "user", "content": "Hi"},            │    │
│      {"role": "assistant", "content": "Hello!"},   │    │
│      {"role": "user", "content": "How are you?"}   │    │
│    ]                                                │    │
│  }                                                  │    │
│                                          System prompt  │
│                                          (optional but  │
│                                           important)    │
└─────────────────────────────────────────────────────────┘
```

### System Prompts

The system prompt sets the **persona and rules** for the assistant:

```json
{
  "system": "You are Johnathan, an AI coding assistant. You help users with programming tasks. Be concise and direct."
}
```

System prompts are:
- Not part of the message array
- Sent with every request
- Where you define behavior, constraints, and personality

### Message Roles

Messages alternate between two roles:

| Role | Who | Purpose |
|------|-----|---------|
| `user` | Human | Questions, requests, feedback |
| `assistant` | Claude | Responses, including tool calls |

**Rule:** Messages must alternate. You can't have two user messages in a row.

### Conversation History

For multi-turn conversations, you send the **entire history** each time:

```
Turn 1: [user: "Hi"]
Turn 2: [user: "Hi", assistant: "Hello!", user: "What's 2+2?"]
Turn 3: [user: "Hi", assistant: "Hello!", user: "What's 2+2?", assistant: "4", user: "Thanks"]
```

The API is **stateless** - it doesn't remember previous calls. Your agent must maintain and send the history.

### Stop Reasons

The response includes why Claude stopped:

| Stop Reason | Meaning |
|-------------|---------|
| `end_turn` | Natural completion |
| `max_tokens` | Hit the token limit |
| `tool_use` | Wants to use a tool (Topic 8) |
| `stop_sequence` | Hit a custom stop sequence |

---

## 4. Implement

**Updated ApiRequest with system prompt:**
```rust
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}
```

**Message constructors:**
```rust
impl Message {
    pub fn user(content: &str) -> Self {
        Self { role: "user".to_string(), content: content.to_string() }
    }

    pub fn assistant(content: &str) -> Self {
        Self { role: "assistant".to_string(), content: content.to_string() }
    }
}
```

**Full-featured send function:**
```rust
pub fn send_messages(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
) -> Result<ChatResponse, String> {
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 4096,
        messages,
        system: system_prompt.map(|s| s.to_string()),
    };
    // ... HTTP call ...
}
```

**REPL with history:**
```rust
fn run_repl(api_key: &str, verbose: bool) {
    let mut history: Vec<Message> = Vec::new();

    loop {
        let input = read_input()?;

        // Add user message to history
        history.push(Message::user(&input));

        // Get response with full history
        let response = eval(history.clone(), api_key, verbose);

        // Add assistant response to history
        history.push(Message::assistant(&response));

        println!("{}\n", response);
    }
}
```

---

## 5. Review

**What we built:**

```
main.rs
├── SYSTEM_PROMPT constant      # Agent's persona
├── run_once()                  # Single message, no history
└── run_repl()
    ├── history: Vec<Message>   # Persists across turns
    ├── history.push(user)      # Add user message
    ├── eval(history.clone())   # Send full history
    └── history.push(assistant) # Add response

api/client.rs
├── Message::user()             # Constructor helpers
├── Message::assistant()
├── send_messages()             # Full API: history + system prompt
└── send_message()              # Convenience wrapper
```

**The conversation flow:**
```
Turn 1: [user: "Hi"] → API → "Hello!"
Turn 2: [user: "Hi", assistant: "Hello!", user: "My name is Bob"] → API
Turn 3: [..., user: "What's my name?"] → API → "Your name is Bob"
```

**Agent concepts demonstrated:**

| Concept | Implementation |
|---------|----------------|
| System prompt | `SYSTEM_PROMPT` constant, passed with every request |
| Message history | `Vec<Message>` in REPL, grows each turn |
| Stateless API | We maintain state, API receives full history each call |
| Stop reason | `ChatResponse.stop_reason` - will matter for tool use |

---

## Key Takeaways

- System prompts define agent persona and rules
- The API is stateless - you must send full conversation history
- Messages must alternate between user and assistant roles
- Stop reasons tell you why the model stopped (important for tool use later)
