# Topic 6: Streaming Responses

## 1. Present

Without streaming, the agent waits for the **entire response** before showing anything. With a long response, the user stares at "Thinking..." for 10+ seconds.

Streaming shows tokens **as they're generated** - the same experience you get in ChatGPT or Claude.ai. Users see the response build in real-time.

Why this matters for agents:
- **Better UX** - Feels responsive, not frozen
- **Early cancellation** - User can Ctrl+C if it's going wrong
- **Tool use detection** - Know immediately when a tool call starts

---

## 2. Relate

Before streaming:
```
User input → API call (wait...) → Full response → Display
            └── 5-10 seconds of "Thinking..." ──┘
```

With streaming:
```
User input → API call → token → token → token → ... → done
                        └── Display each token as it arrives ──┘
```

---

## 3. Explain

### Server-Sent Events (SSE)

Instead of one response, the server sends a **stream of events**:

```
event: message_start
data: {"type": "message_start", "message": {...}}

event: content_block_delta
data: {"type": "content_block_delta", "delta": {"text": "Hello"}}

event: content_block_delta
data: {"type": "content_block_delta", "delta": {"text": " world"}}

event: message_stop
data: {"type": "message_stop"}
```

### Event Types

| Event | Meaning |
|-------|---------|
| `message_start` | Response beginning, contains metadata |
| `content_block_start` | A content block (text or tool_use) starting |
| `content_block_delta` | Incremental content (the actual tokens) |
| `content_block_stop` | Content block finished |
| `message_delta` | Final stats (stop_reason, usage) |
| `message_stop` | Stream complete |

### The Request Change

Enable streaming by adding `"stream": true`:

```json
{
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 4096,
  "stream": true,
  "messages": [...]
}
```

### Why Streaming Matters for Tool Use

When Claude wants to use a tool, you'll see:
```
content_block_start: {"type": "tool_use", "name": "read_file"}
```

With streaming, you can show "Reading file..." immediately.

---

## 4. Implement

**api/client.rs - Streaming function:**
```rust
pub fn send_messages_streaming<F>(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
    mut on_chunk: F,
) -> Result<ChatResponse, String>
where
    F: FnMut(&str),
{
    let request = ApiRequest {
        // ... other fields ...
        stream: true,
    };

    let response = client.post(API_URL)
        .json(&request)
        .send()?;

    // Read SSE stream line by line
    let reader = BufReader::new(response);

    for line in reader.lines() {
        let line = line?;

        // SSE format: "data: {json}"
        if let Some(data) = line.strip_prefix("data: ") {
            // Parse content_block_delta events
            if let Ok(event) = serde_json::from_str::<ContentBlockDelta>(data) {
                if let Some(text) = event.delta.and_then(|d| d.text) {
                    on_chunk(&text);  // Call the callback
                    full_text.push_str(&text);
                }
            }
        }
    }

    Ok(ChatResponse { text: full_text, stop_reason })
}
```

**main.rs - Using streaming:**
```rust
fn eval_streaming(messages: Vec<Message>, api_key: &str, verbose: bool) -> String {
    print!("Thinking...");
    io::stdout().flush().ok();

    let mut first_chunk = true;

    let result = api::send_messages_streaming(
        api_key,
        messages,
        Some(SYSTEM_PROMPT),
        |chunk| {
            // Clear "Thinking..." on first chunk
            if first_chunk {
                print!("\r            \r");
                first_chunk = false;
            }
            // Print chunk immediately
            print!("{}", chunk);
            io::stdout().flush().ok();
        },
    );

    // Return full text for history
    result.map(|r| r.text).unwrap_or_default()
}
```

---

## 5. Review

**The streaming flow:**

```
eval_streaming()
├── print!("Thinking...")
├── send_messages_streaming(..., |chunk| {
│       if first_chunk: clear "Thinking..."
│       print!("{}", chunk)
│   })
└── return full text for history

send_messages_streaming()
├── Request with stream: true
├── BufReader for line-by-line SSE
├── Parse "data: {json}" lines
├── Extract text from content_block_delta
└── Call on_chunk() for each piece
```

**Agent concepts:**

| Concept | Why It Matters |
|---------|----------------|
| Streaming | Responsive UX, real-time output |
| SSE parsing | Standard format for streaming APIs |
| Callback pattern | Process chunks as they arrive |
| Early feedback | Clear "Thinking..." on first token |

---

## Key Takeaways

- Streaming makes the agent feel responsive
- SSE is the standard format: `data: {json}` lines
- Callbacks let you process each chunk as it arrives
- Clear loading indicators as soon as real content starts
