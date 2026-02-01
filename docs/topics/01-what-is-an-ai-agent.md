# Topic 1: What is an AI Agent?

## 1. Present

The word "agent" gets thrown around a lot, but it has a specific meaning that separates it from a regular chatbot.

**A chatbot** takes input and produces output. That's it. It's a function: `input -> output`.

**An agent** operates in a loop. It can:
- Observe the environment
- Decide what action to take
- Execute that action
- See the result
- Repeat until the goal is achieved

The key difference: **agents can act on the world**, not just talk about it.

---

## 2. Relate

A basic program does one thing and exits:

```rust
fn main() {
    println!("Hello, world!");
}
```

By the end of this project, `main.rs` will instead:
1. Start a loop
2. Read your input
3. Send it to an LLM
4. Let the LLM request tools (read files, run commands, etc.)
5. Execute those tools
6. Return results to the LLM
7. Repeat until the task is done

That transformation - from "run once and exit" to "loop until goal achieved" - is what makes it an agent.

---

## 3. Explain

### The Agent Mental Model

Think of an agent like a contractor you hired:

| Chatbot | Agent |
|---------|-------|
| "You should probably fix that pipe" | *Actually fixes the pipe* |
| "Here's how to write that function" | *Writes the function to your file* |
| "Try running npm install" | *Runs npm install and tells you what happened* |

### The Core Components

Every agent has these pieces:

```
┌─────────────────────────────────────────────────┐
│                   AGENT                         │
│                                                 │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐     │
│  │  Brain  │───▶│ Tools   │───▶│ World   │     │
│  │  (LLM)  │◀───│         │◀───│         │     │
│  └─────────┘    └─────────┘    └─────────┘     │
│                                                 │
│  Brain: Decides what to do                      │
│  Tools: How it acts (read file, run cmd, etc)  │
│  World: The environment it operates in          │
└─────────────────────────────────────────────────┘
```

### The Agent Loop (Preview of Topic 2)

```
while goal_not_achieved:
    observation = perceive(environment)
    thought = llm.think(observation + history)
    action = thought.get_action()
    result = execute(action)
    history.append(observation, thought, action, result)
```

This loop is the heartbeat. Everything else we build serves this loop.

---

## 4. Implement

The initial skeleton with agent loop structure:

```rust
fn main() {
    let goal_achieved = false;

    // THE AGENT LOOP - the core pattern
    while !goal_achieved {
        // 1. OBSERVE - get input from user or environment
        let observation = observe();

        // 2. THINK - send to LLM, get back a decision
        let thought = think(&observation);

        // 3. ACT - execute the decided action
        let result = act(&thought);

        // 4. CHECK - is our goal achieved?
        break; // Remove once we have real logic
    }
}

fn observe() -> String {
    "User wants to understand what an agent is".to_string()
}

fn think(observation: &str) -> String {
    "I should explain the agent concept".to_string()
}

fn act(thought: &str) -> String {
    "Action completed".to_string()
}
```

---

## 5. Review

Structure:

```
main()
├── while !goal_achieved     <-- THE LOOP (this is what makes it an agent)
│   ├── observe()            <-- Step 1: What's happening?
│   ├── think()              <-- Step 2: What should I do?
│   ├── act()                <-- Step 3: Do it
│   └── (check goal)         <-- Step 4: Am I done?
```

**Key insight**: The structure is already there. The functions are stubs, but the *pattern* is correct. Over the coming topics, we'll replace each stub with real functionality:

| Function | Now | Later |
|----------|-----|-------|
| `observe()` | Returns hardcoded string | Reads user input, file states |
| `think()` | Returns hardcoded string | Calls Claude API, parses response |
| `act()` | Returns hardcoded string | Executes tools (files, commands) |

---

## Key Takeaways

- An agent is defined by its loop (not just responding, but acting repeatedly)
- The pattern: observe -> think -> act -> check goal -> repeat
- Everything we build serves this central loop
