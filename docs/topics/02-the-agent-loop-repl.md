# Topic 2: The Agent Loop (REPL Pattern)

## 1. Present

REPL stands for **Read-Eval-Print-Loop**. It's the pattern behind every interactive shell you've used:
- Python interpreter
- Node.js console
- Bash itself
- And every agent CLI

The agent loop is a specialized REPL where "Eval" involves calling an LLM and potentially executing tools.

---

## 2. Relate

The initial code had the loop structure but wasn't interactive:

```rust
while !goal_achieved {
    let observation = observe();  // Hardcoded string
    let thought = think(&observation);
    let result = act(&thought);
    break;  // Exits immediately
}
```

We need to make this actually read from the user, respond, and continue until they quit.

---

## 3. Explain

### The Classic REPL

```
┌──────────────────────────────────────┐
│              REPL                    │
│                                      │
│   ┌──────┐                           │
│   │ READ │◄── User types input       │
│   └──┬───┘                           │
│      ▼                               │
│   ┌──────┐                           │
│   │ EVAL │◄── Process the input      │
│   └──┬───┘                           │
│      ▼                               │
│   ┌───────┐                          │
│   │ PRINT │◄── Show the result       │
│   └──┬────┘                          │
│      ▼                               │
│   ┌──────┐                           │
│   │ LOOP │◄── Go back to READ        │
│   └──────┘                           │
└──────────────────────────────────────┘
```

### The Agent REPL (more complex)

```
┌─────────────────────────────────────────────────────────┐
│                    AGENT LOOP                           │
│                                                         │
│   ┌──────┐                                              │
│   │ READ │◄── User input OR tool result                 │
│   └──┬───┘                                              │
│      ▼                                                  │
│   ┌──────┐    ┌─────────────────┐                       │
│   │ EVAL │───▶│ Call LLM        │                       │
│   └──┬───┘    │ Parse response  │                       │
│      │        │ Extract action  │                       │
│      │        └─────────────────┘                       │
│      ▼                                                  │
│   ┌───────┐   Was it a tool call?                       │
│   │ PRINT │   ├─ Yes: Execute tool, feed result back   │
│   └──┬────┘   └─ No: Show response to user             │
│      ▼                                                  │
│   ┌──────┐                                              │
│   │ LOOP │◄── Continue until user quits                 │
│   └──────┘                                              │
└─────────────────────────────────────────────────────────┘
```

The key difference: the agent loop has an **inner loop** for tool execution. The LLM might request multiple tools before giving a final response.

### Why This Matters

Without the loop, you'd have to manually copy-paste between the LLM and your terminal. The loop automates this back-and-forth, making the agent autonomous.

---

## 4. Implement

```rust
use std::io::{self, Write};

fn main() {
    println!("Johnathan Agent v0.1.0");
    println!("Type 'quit' or 'exit' to stop.\n");

    // THE AGENT LOOP (REPL Pattern)
    loop {
        // 1. READ - Get input from the user
        let input = match read_input() {
            Some(input) => input,
            None => continue,
        };

        // Check for exit commands
        if should_exit(&input) {
            println!("Goodbye!");
            break;
        }

        // 2. EVAL - Process the input (later: send to LLM)
        let response = eval(&input);

        // 3. PRINT - Show the result
        println!("{}\n", response);

        // 4. LOOP - automatically continues
    }
}

fn read_input() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let trimmed = input.trim().to_string();
    if trimmed.is_empty() { None } else { Some(trimmed) }
}

fn should_exit(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower == "quit" || lower == "exit" || lower == "q"
}

fn eval(input: &str) -> String {
    // Future: call LLM, handle tool use
    format!("[Echo] You said: {}", input)
}
```

---

## 5. Review

Structure:

```
main()
└── loop                    <-- Infinite loop (the REPL)
    ├── read_input()        <-- READ: get user input
    ├── should_exit()       <-- Check for quit
    ├── eval()              <-- EVAL: process (currently echoes)
    └── println!()          <-- PRINT: show result
```

**The roadmap for `eval()`:**

```rust
// 1. Add user message to conversation history
// 2. Send history to LLM
// 3. Get response
// 4. If response contains tool call:
//    a. Execute tool
//    b. Add tool result to history
//    c. Go back to step 2 (inner loop!)
// 5. Return final text response
```

That inner loop (steps 4a-4c) is what makes an agent different from a simple chatbot.

---

## Key Takeaways

- REPL = Read-Eval-Print-Loop
- The agent loop has an inner loop for tool execution
- Without the loop, you'd manually copy-paste between LLM and terminal
