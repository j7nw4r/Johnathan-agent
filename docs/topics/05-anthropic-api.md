# Topic 5: The Anthropic API

## The Concept

The Anthropic API has specific conventions for conversations:

- **System prompts** define the AI's persona and rules
- **Message history** maintains conversation context
- **Role alternation** enforces user/assistant turn-taking

## System Prompts

A system prompt tells Claude *who it is* and *how to behave*:

```rust
const SYSTEM_PROMPT: &str = r#"You are Johnathan, an AI coding assistant.

You help users with programming tasks. Be concise and direct.
When asked to perform tasks, explain what you're doing briefly.

You are running as a CLI agent and can have multi-turn conversations."#;
```

System prompts are sent with every request but aren't part of the message history—they're a separate field:

```json
{
  "model": "claude-sonnet-4-20250514",
  "system": "You are Johnathan...",
  "messages": [...]
}
```

## Message History and Roles

### The Stateless API

LLM APIs don't maintain sessions. Each request must include the full conversation:

```
Request 1: [user: "Hi"]
Response 1: "Hello!"

Request 2: [user: "Hi", assistant: "Hello!", user: "What's 2+2?"]
Response 2: "4"

Request 3: [user: "Hi", assistant: "Hello!", user: "What's 2+2?", assistant: "4", user: "Double it"]
Response 3: "8"
```

### Role Alternation

Anthropic requires strict alternation between `user` and `assistant` roles:

```
✓ user → assistant → user → assistant
✗ user → user (error!)
✗ assistant → assistant (error!)
```

This means after every assistant response, we must wait for user input before sending another request.

## Code Implementation

### Sending System Prompt

```rust
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,  // System prompt goes here, separate from messages
}
```

The `#[serde(skip_serializing_if = "Option::is_none")]` prevents sending `"system": null` when there's no system prompt.

### Maintaining History in REPL

```rust
fn run_repl(api_key: &str, verbose: bool) {
    let mut history: Vec<Message> = Vec::new();

    loop {
        let input = read_input()?;

        // Add user message
        history.push(Message::user(&input));

        // Send full history to API
        let response = send_messages(
            api_key,
            history.clone(),      // Clone because we need to keep original
            Some(SYSTEM_PROMPT),
        )?;

        // Add assistant response to history
        history.push(Message::assistant(&response));

        println!("{}", response);
    }
}
```

### Non-Interactive Mode (Single Turn)

For one-shot queries, history is just one message:

```rust
fn run_once(prompt: &str, api_key: &str) {
    let messages = vec![Message::user(prompt)];
    let response = send_messages(api_key, messages, Some(SYSTEM_PROMPT));
    println!("{}", response);
}
```

## Best Practices

### System Prompt Design

Good system prompts:
- Define the persona clearly
- Set behavioral boundaries
- Mention capabilities and limitations
- Are concise (tokens cost money)

```rust
// Good: Clear, specific, concise
"You are a coding assistant. Be direct. Explain briefly what you do."

// Bad: Vague, verbose
"You are a helpful AI assistant who tries to help users with
various tasks and always aims to be as helpful as possible..."
```

### History Management

For long conversations, history grows and:
- Uses more tokens (costs more)
- May hit context limits
- Slows down responses

Topic 14 covers strategies for managing this (summarization, truncation).

## Key Takeaways

1. **System prompt = persona** - Separate from messages, sent every request
2. **History = context** - Full conversation sent each time (stateless API)
3. **Role alternation** - Must alternate user/assistant
4. **Clone history** - Don't consume it when sending, you need it for next turn
5. **Design prompts carefully** - Clear, specific, concise

## What's Next

Topic 6 adds streaming responses, showing tokens as they arrive instead of waiting for the full response.
