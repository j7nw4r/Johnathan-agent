# Topic 2: The Agent Loop (REPL Pattern)

## The Concept

A REPL is a classic pattern in programming: **Read-Eval-Print-Loop**. It's how interactive interpreters work (Python, Node, etc.), and it maps perfectly to our agent loop:

| REPL Step | Agent Step | What Happens |
|-----------|------------|--------------|
| **Read**  | Observe    | Get user input |
| **Eval**  | Think+Act  | Process with LLM, execute tools |
| **Print** | Report     | Show results to user |
| **Loop**  | Check      | Continue until exit |

## Two Modes of Operation

Our agent supports two modes:

### Interactive Mode (REPL)
```
$ johnathan
> What files are in this directory?
[agent reads directory, responds]
> Now create a new file called hello.txt
[agent creates file, responds]
> quit
Goodbye!
```

The user has a conversation. The agent maintains context across turns.

### Non-Interactive Mode (Single Prompt)
```
$ johnathan "What files are in this directory?"
[agent responds and exits]
```

For scripting and quick queries. No ongoing conversation.

## Code Implementation

### The REPL Structure

```rust
// src/main.rs

fn run_repl(api_key: &str, verbose: bool) {
    println!("Type 'quit' or 'exit' to stop.\n");

    // Message history persists across the loop
    let mut history: Vec<Message> = Vec::new();

    loop {
        // READ: Get user input
        let input = match read_input() {
            Some(input) => input,
            None => continue,  // Empty input, try again
        };

        // Check for exit commands
        if should_exit(&input) {
            println!("Goodbye!");
            break;
        }

        // Add user message to history
        history.push(Message::user(&input));

        // EVAL: Send to LLM and get response
        let response = eval(history.clone(), api_key);

        // Add assistant response to history (for context in next turn)
        history.push(Message::assistant(&response));

        // PRINT: Display the response
        println!("{}\n", response);

        // LOOP: Continue (implicit via loop {})
    }
}
```

### Reading Input

```rust
fn read_input() -> Option<String> {
    // Show prompt
    print!("> ");
    io::stdout().flush().ok()?;

    // Read line from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    // Return trimmed input, or None if empty
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
```

### Exit Detection

```rust
fn should_exit(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower == "quit" || lower == "exit" || lower == "q"
}
```

## Message History

A critical concept: **LLM APIs are stateless**. Each request must include the full conversation history. This is why we maintain `history: Vec<Message>`:

```
Turn 1:
  User: "What's 2+2?"
  Send: [user: "What's 2+2?"]
  Recv: "4"

Turn 2:
  User: "Double that"
  Send: [user: "What's 2+2?", assistant: "4", user: "Double that"]
  Recv: "8"
```

Without history, the LLM wouldn't know what "that" refers to.

## Visual Feedback

Users need to know the agent is working:

```rust
fn eval(messages: Vec<Message>, api_key: &str) -> String {
    // Show that we're processing
    print!("Thinking...");
    io::stdout().flush().ok();

    // ... make API call ...

    // Clear the indicator
    print!("\r            \r");

    // Return response
}
```

## Key Takeaways

1. **REPL maps to agent loop** - Read=Observe, Eval=Think+Act, Print=Report
2. **History is essential** - LLM APIs are stateless; we maintain context
3. **Two modes** - Interactive for conversations, non-interactive for scripts
4. **User feedback matters** - Show the user something is happening

## What's Next

Topic 3 adds proper CLI argument parsing with clap, making our two modes accessible via command-line flags.
