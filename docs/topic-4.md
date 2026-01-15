# Topic 4: HTTP Requests and API Basics

## 1. Present

To make an agent intelligent, we need to send messages to an LLM and get responses. This happens over HTTP - the same protocol your browser uses.

Understanding HTTP is essential because:
- Every LLM API (Claude, OpenAI, etc.) uses HTTP
- Tool implementations often need HTTP (web search, APIs)
- Debugging agent issues often means understanding HTTP requests/responses

This is where the agent gets its "brain."

---

## 2. Relate

The `eval()` function was a stub:

```rust
fn eval(input: &str, verbose: bool) -> String {
    print!("Thinking...");
    std::thread::sleep(std::time::Duration::from_millis(500));
    format!("[Echo] {}", input)  // <-- This becomes an API call
}
```

We replace the sleep and echo with a real HTTP request to Claude's API.

---

## 3. Explain

### The HTTP Request/Response Cycle

```
┌─────────────────────────────────────────────────────────┐
│                    HTTP Cycle                           │
│                                                         │
│  Agent                                    Claude API    │
│    │                                           │        │
│    │──── POST /v1/messages ──────────────────▶│        │
│    │     Headers: Authorization, Content-Type │        │
│    │     Body: { messages, model, ... }       │        │
│    │                                           │        │
│    │◀─── 200 OK ─────────────────────────────│        │
│    │     Body: { content, stop_reason, ... }  │        │
│    │                                           │        │
└─────────────────────────────────────────────────────────┘
```

### Key Concepts

**HTTP Methods:**
- `GET` - retrieve data (read-only)
- `POST` - send data to create/process something (what we use for LLM APIs)

**Headers:** Metadata about the request
- `x-api-key: sk-ant-...` - your API key
- `anthropic-version: 2023-06-01` - API version
- `content-type: application/json` - we're sending JSON

**Request Body:** The actual data (JSON for Claude API)
```json
{
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 1024,
  "messages": [
    {"role": "user", "content": "Hello!"}
  ]
}
```

**Response:** What comes back
```json
{
  "content": [{"type": "text", "text": "Hello! How can I help?"}],
  "stop_reason": "end_turn",
  "usage": {"input_tokens": 10, "output_tokens": 15}
}
```

### Why JSON?

JSON is the universal language of APIs:
- Human readable
- Easy to parse in any language
- Maps naturally to data structures (objects, arrays)

The flow: **Rust struct → serialize to JSON → HTTP → JSON response → deserialize to Rust struct**

---

## 4. Implement

**Cargo.toml:**
```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**src/api/client.rs:**

```rust
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
    stop_reason: Option<String>,
}

pub fn send_message(api_key: &str, user_message: &str) -> Result<String, String> {
    // 1. Build request
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        }],
    };

    // 2. Send HTTP POST
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // 3. Check status
    if !response.status().is_success() {
        return Err(format!("API error {}", response.status()));
    }

    // 4. Parse JSON
    let api_response: ApiResponse = response.json()?;

    // 5. Extract text
    let text = api_response.content
        .iter()
        .filter_map(|block| block.text.clone())
        .collect::<Vec<_>>()
        .join("");

    Ok(text)
}
```

---

## 5. Review

**The HTTP flow in our agent:**

```
eval()
└── api::send_message(api_key, input)
    ├── Build ApiRequest struct
    ├── Serialize to JSON
    ├── POST to https://api.anthropic.com/v1/messages
    │   └── Headers: x-api-key, anthropic-version, content-type
    ├── Check response.status()
    ├── Deserialize JSON → ApiResponse
    └── Extract text from content blocks
```

**Agent concepts demonstrated:**

| Concept | Why It Matters |
|---------|----------------|
| HTTP POST | LLM APIs receive data, not just retrieve it |
| Headers | Authentication and versioning |
| JSON serialization | Convert between code structures and wire format |
| Error handling | Network calls fail; handle gracefully |

---

## Key Takeaways

- LLM APIs use HTTP POST with JSON bodies
- Headers carry authentication and metadata
- Serialize structs to JSON, deserialize responses back
- Always handle network errors gracefully
