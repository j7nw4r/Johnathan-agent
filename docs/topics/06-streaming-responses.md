# Topic 6: Streaming Responses

## The Concept

Without streaming, the user sees nothing while the LLM generates a response—potentially many seconds of silence. Streaming shows tokens as they're generated, providing immediate feedback.

**Without streaming:**
```
> Tell me about Rust
[5 seconds of nothing]
Rust is a systems programming language...
```

**With streaming:**
```
> Tell me about Rust
Rust is a sy|stems prog|ramming la|nguage...
       ↑ tokens appear progressively
```

## Server-Sent Events (SSE)

Anthropic uses SSE for streaming. Instead of one JSON response, you receive a stream of events:

```
event: message_start
data: {"type":"message_start","message":{"id":"msg_01..."}}

event: content_block_start
data: {"type":"content_block_start","index":0,"content_block":{"type":"text"}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}

event: content_block_delta
data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":" there"}}

event: message_stop
data: {"type":"message_stop"}
```

Each `content_block_delta` contains a piece of the response.

## Code Implementation

### Enabling Streaming

Add `stream: true` to the request:

```rust
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    stream: bool,  // Enable streaming
}
```

### Parsing the Stream

```rust
use std::io::{BufRead, BufReader};

pub fn send_messages_streaming<F>(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
    mut on_chunk: F,  // Callback for each text chunk
) -> Result<String, String>
where
    F: FnMut(&str),
{
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 4096,
        messages,
        system: system_prompt.map(|s| s.to_string()),
        stream: true,
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .json(&request)
        .send()
        .map_err(|e| e.to_string())?;

    // Read response line by line
    let reader = BufReader::new(response);
    let mut full_text = String::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;

        // SSE format: "data: {...}"
        if let Some(data) = line.strip_prefix("data: ") {
            // Parse delta events
            if let Ok(delta) = serde_json::from_str::<StreamDelta>(data) {
                if let Some(text) = delta.text {
                    on_chunk(&text);          // Call callback immediately
                    full_text.push_str(&text); // Accumulate for return
                }
            }
        }
    }

    Ok(full_text)
}
```

### The Callback Pattern

The `on_chunk` callback lets the caller decide what to do with each piece:

```rust
// Print immediately as chunks arrive
let response = send_messages_streaming(
    api_key,
    messages,
    Some(SYSTEM_PROMPT),
    |chunk| {
        print!("{}", chunk);      // No newline - text flows continuously
        io::stdout().flush().ok(); // Ensure it displays immediately
    },
)?;
```

### Handling the "Thinking..." Indicator

Clear the waiting message when the first chunk arrives:

```rust
fn eval_streaming(messages: Vec<Message>, api_key: &str) -> String {
    print!("Thinking...");
    io::stdout().flush().ok();

    let mut first_chunk = true;

    let result = send_messages_streaming(
        api_key,
        messages,
        Some(SYSTEM_PROMPT),
        |chunk| {
            if first_chunk {
                // Overwrite "Thinking..." with spaces, return to start of line
                print!("\r            \r");
                io::stdout().flush().ok();
                first_chunk = false;
            }
            print!("{}", chunk);
            io::stdout().flush().ok();
        },
    );

    result.unwrap_or_else(|e| format!("Error: {}", e))
}
```

## SSE Event Types

The stream contains different event types:

| Event | Purpose |
|-------|---------|
| `message_start` | Beginning of response, contains message ID |
| `content_block_start` | New content block starting (text, tool use) |
| `content_block_delta` | Piece of content (text chunk, tool input JSON) |
| `content_block_stop` | Content block complete |
| `message_delta` | Message-level updates (stop_reason) |
| `message_stop` | End of response |

For basic text streaming, we only need `content_block_delta` with `text_delta` type.

## Key Takeaways

1. **Streaming = immediate feedback** - Users see progress, not silence
2. **SSE format** - Lines starting with `data: ` contain JSON events
3. **Callback pattern** - Pass a function to handle each chunk
4. **Flush stdout** - Required for immediate display in terminal
5. **Accumulate text** - Build full response while streaming for history

## What's Next

Topic 8 introduces tool use—how Claude can request to run tools, and how we parse those requests from the stream.
