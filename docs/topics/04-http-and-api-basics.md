# Topic 4: HTTP Requests and API Basics

## The Concept

To use an LLM, we need to communicate with it over HTTP. This topic covers:

- How HTTP requests work (POST, headers, body)
- JSON serialization/deserialization
- Making requests with the `reqwest` crate
- Handling responses and errors

## HTTP Fundamentals for APIs

Most LLM APIs use this pattern:

```
POST /v1/messages HTTP/1.1
Host: api.anthropic.com
Content-Type: application/json
x-api-key: sk-ant-...

{
  "model": "claude-sonnet-4-20250514",
  "messages": [{"role": "user", "content": "Hello"}],
  "max_tokens": 1024
}
```

Key parts:
- **POST** - We're sending data, not just requesting
- **Headers** - Authentication and content type
- **Body** - JSON with our request details

## Dependencies

```toml
# Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

- `reqwest` - HTTP client
- `serde` - Serialization framework
- `serde_json` - JSON support

## Code Implementation

### Request/Response Types

```rust
use serde::{Deserialize, Serialize};

// What we send to the API
#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

// What we get back (simplified)
#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    text: String,
}
```

The `#[derive(Serialize)]` and `#[derive(Deserialize)]` macros auto-generate JSON conversion code.

### Making the Request

```rust
const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

pub fn send_messages(
    api_key: &str,
    messages: Vec<Message>,
    system_prompt: Option<&str>,
) -> Result<String, String> {
    // Build request body
    let request = ApiRequest {
        model: "claude-sonnet-4-20250514".to_string(),
        max_tokens: 4096,
        messages,
        system: system_prompt.map(|s| s.to_string()),
    };

    // Create HTTP client and send
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&request)  // Serializes to JSON automatically
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // Check for HTTP errors
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    // Parse JSON response
    let api_response: ApiResponse = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Extract text from first content block
    Ok(api_response.content
        .first()
        .map(|c| c.text.clone())
        .unwrap_or_default())
}
```

### Message Types

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(text: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: text.to_string(),
        }
    }

    pub fn assistant(text: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: text.to_string(),
        }
    }
}
```

## Error Handling Pattern

We use `Result<T, String>` for simplicity, converting errors to readable messages:

```rust
.send()
.map_err(|e| format!("HTTP request failed: {}", e))?;
```

The `?` operator propagates errors up the call stack.

## Key Takeaways

1. **POST with JSON** - LLM APIs receive JSON, return JSON
2. **Headers matter** - API key and version are required
3. **Serde derives** - `Serialize`/`Deserialize` auto-generate JSON code
4. **Result for errors** - Propagate errors with `?`, convert with `map_err`
5. **Blocking client** - We use sync HTTP for simplicity (async comes later)

## What's Next

Topic 5 dives deeper into the Anthropic API structure, covering system prompts and proper message history management.
