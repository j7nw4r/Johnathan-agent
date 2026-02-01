# Topic 1: What is an AI Agent?

## The Core Concept

An AI agent is fundamentally different from a chatbot. While a chatbot takes input and produces output once, an agent **loops until the goal is achieved**.

Think of the difference between asking someone for advice versus hiring a contractor:
- **Chatbot**: "Here's how you could fix that bug..." (gives advice, done)
- **Agent**: *Actually fixes the bug* - reads files, makes changes, runs tests, iterates

## The Three Components

Every AI agent has three core components:

```
┌─────────────────────────────────────────────────────┐
│                     AI AGENT                        │
│                                                     │
│   ┌─────────┐    ┌─────────┐    ┌─────────┐        │
│   │  Brain  │    │  Tools  │    │  World  │        │
│   │  (LLM)  │───▶│(actions)│───▶│(environ)│        │
│   └─────────┘    └─────────┘    └─────────┘        │
│                                                     │
└─────────────────────────────────────────────────────┘
```

1. **Brain (LLM)**: The reasoning engine - decides what to do next
2. **Tools**: Actions the agent can take (read files, run commands, etc.)
3. **World/Environment**: What the agent operates on (filesystem, APIs, etc.)

## The Agent Loop

The defining characteristic of an agent is its loop:

```
┌──────────────────────────────────────┐
│                                      │
│   ┌─────────┐                        │
│   │ Observe │◀───────────────────┐   │
│   └────┬────┘                    │   │
│        │                         │   │
│        ▼                         │   │
│   ┌─────────┐                    │   │
│   │  Think  │                    │   │
│   └────┬────┘                    │   │
│        │                         │   │
│        ▼                         │   │
│   ┌─────────┐    ┌─────────┐     │   │
│   │   Act   │───▶│  Check  │─────┘   │
│   └─────────┘    │  Done?  │         │
│                  └────┬────┘         │
│                       │ yes          │
│                       ▼              │
│                     EXIT             │
└──────────────────────────────────────┘
```

1. **Observe**: Gather information (read user input, check file contents, etc.)
2. **Think**: Decide what to do next (this is where the LLM reasons)
3. **Act**: Execute the decision (run a tool, make a change)
4. **Check**: Is the goal achieved? If not, loop back

## Code Implementation

In our agent, this loop is represented as a simple structure:

```rust
// src/main.rs - The skeleton we started with

fn main() {
    loop {
        // OBSERVE: Get input from the world
        let observation = observe();

        // THINK: Decide what to do (LLM call)
        let action = think(observation);

        // ACT: Execute the decision
        let result = act(action);

        // CHECK: Are we done?
        if is_goal_achieved(result) {
            break;
        }
    }
}
```

This skeleton evolves throughout the project, but the core pattern remains.

## Key Takeaways

1. **Agents loop** - They don't just respond once; they iterate until done
2. **Agents act** - They don't just advise; they make changes in the world
3. **The loop is central** - Everything we build serves this observe→think→act→check cycle
4. **LLM is the brain** - It decides *what* to do; tools handle *how* to do it

## What's Next

Topic 2 will implement this loop as a REPL (Read-Eval-Print-Loop), giving us an interactive interface to work with.
