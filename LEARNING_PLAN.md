# Agent CLI Learning Plan

This document outlines the step-by-step journey to build an AI agent CLI (similar to Claude Code or Codex) while learning the underlying concepts.

## Learning Philosophy

For each topic, we follow this structure:
1. **Present** - Introduce the concept and why it matters
2. **Relate** - Show how it connects to our codebase
3. **Explain** - Deep dive into the theory
4. **Implement** - Write the code together
5. **Review** - Walk through the implementation and discuss

---

## Phase 1: Foundations

### Topic 1: What is an AI Agent?
Understanding the difference between a chatbot and an agent. Agents can *act* on the world through tools, not just respond.

### Topic 2: The Agent Loop (REPL Pattern)
The core architecture: Read input -> Think -> Act -> Observe -> Repeat. This is the heartbeat of any agent system.

### Topic 3: CLI Basics in Rust
Setting up a proper command-line interface with argument parsing, colored output, and user input handling.

---

## Phase 2: LLM Integration

### Topic 4: HTTP Requests and API Basics
How to communicate with external services. Understanding HTTP, JSON, and REST APIs.

### Topic 5: The Anthropic API
Structure of messages, system prompts, and how to make your first API call to Claude.

### Topic 6: Streaming Responses
Why streaming matters for UX, and how to handle Server-Sent Events (SSE) for real-time output.

### Topic 7: Async Rust Fundamentals
Understanding tokio, async/await, and why async matters for I/O-bound operations like API calls.

---

## Phase 3: Tool Use (The Heart of Agents)

### Topic 8: What is Tool Use / Function Calling?
How LLMs can request to use tools, and how this enables agents to act on the world.

### Topic 9: Designing a Tool System
Creating a flexible, extensible architecture for tools. Traits, registration, and execution.

### Topic 10: Implementing Core Tools
Building essential tools: file read, file write, bash execution, and search.

### Topic 11: The Tool Use Loop
Handling the back-and-forth: LLM requests tool -> Execute tool -> Return result -> LLM continues.

---

## Phase 4: Context and Memory

### Topic 12: Message History Management
Maintaining conversation context, token limits, and summarization strategies.

### Topic 13: System Prompts and Persona
Crafting effective system prompts that guide agent behavior.

### Topic 14: Context Window Strategies
Handling large codebases when context is limited. Chunking, relevance, and smart retrieval.

---

## Phase 5: Production Readiness

### Topic 15: Error Handling and Recovery
Graceful degradation, retries, and user-friendly error messages.

### Topic 16: Configuration Management
API keys, user preferences, and persistent settings.

### Topic 17: Safety and Sandboxing
Protecting the user from dangerous operations. Confirmation prompts and restricted execution.

### Topic 18: Testing an Agent
How to test non-deterministic AI systems. Mocking, evaluation, and regression testing.

---

## Phase 6: Advanced Features

### Topic 19: Multi-turn Planning
How agents can break down complex tasks and maintain focus across many turns.

### Topic 20: Conversation Persistence
Saving and resuming sessions. Serialization and state management.

### Topic 21: Performance Optimization
Parallel tool execution, caching, and minimizing latency.

### Topic 22: User Experience Polish
Progress indicators, keyboard shortcuts, and terminal UI best practices.

---

## Stretch Goals (Optional)

- **Topic 23**: Sub-agents and delegation
- **Topic 24**: MCP (Model Context Protocol) integration
- **Topic 25**: IDE/Editor integration
- **Topic 26**: Custom tool creation by users

---

## How to Use This Plan

1. We work through topics in order (dependencies build on each other)
2. Check off topics in `TOPICS.md` as we complete them
3. The AI journal (`AI_JOURNAL.md`) tracks our progress and context
4. Feel free to ask to dive deeper or skip ahead on any topic
5. New topics can be added as we discover them

Let's build something great!
