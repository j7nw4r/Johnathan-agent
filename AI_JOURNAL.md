# AI Journal - Johnathan Agent Project

This journal maintains context across sessions. It serves as the "teacher's memory" to ensure continuity in our learning journey.

---

## Project Overview
- **Project Name**: Johnathan-agent
- **Goal**: Build an AI agent CLI (similar to Claude Code/Codex) while learning the underlying concepts
- **Language**: Rust
- **Started**: January 2026

---

## Session Log

### Session 1 - January 15, 2026

**What we did:**
- Reviewed the existing codebase (fresh Rust project with "Hello, world!")
- Created the learning plan (`LEARNING_PLAN.md`) with 26 topics across 6 phases
- Created the topics checklist (`TOPICS.md`) for tracking progress
- Established this journal for maintaining context

**Current state of codebase:**
- `src/main.rs`: Agent loop skeleton with observe/think/act stubs
- `Cargo.toml`: Empty dependencies, edition 2024
- Tracking files: LEARNING_PLAN.md, TOPICS.md, AI_JOURNAL.md, CLAUDE.md

**Next steps:**
- Topic 3: CLI Basics (polish) OR Topic 4: HTTP/API (connect to LLM)
- User preference: skip CLI polish if eager to connect to Claude

**User preferences noted:**
- Wants step-by-step educational approach
- Wants to understand concepts, not just have working code
- Interested in the "why" behind design decisions
- Following along in vim in a separate tmux pane
- Focus on AI agent concepts only, not Rust language concepts
- Commit after each topic
- Don't narrate housekeeping/note-taking

**Open questions:**
- None yet

---

### Topic 1 Completed: What is an AI Agent?

**Key concepts taught:**
- Difference between chatbot (input -> output) and agent (loop until goal)
- The contractor analogy: agents *do* things, not just advise
- Core components: Brain (LLM), Tools, World
- The agent loop pattern: observe -> think -> act -> check

**Code implemented:**
- `src/main.rs` - Skeleton with agent loop structure
- Three stub functions: `observe()`, `think()`, `act()`
- While loop that will become the real agent loop

**What the user should understand:**
- An agent is defined by its ability to act in a loop
- The loop continues until the goal is achieved
- Everything we build serves this central loop

---

## Key Decisions Made
<!-- Track important architectural and design decisions here -->

1. *No decisions made yet - starting fresh*

---

## Concepts the User Has Learned
<!-- Track what concepts have been explained and understood -->

1. **Agent vs Chatbot** - An agent loops and acts; a chatbot responds once
2. **The Agent Loop** - observe -> think -> act -> check (repeat)
3. **Core Components** - Brain (LLM), Tools, World/Environment
4. **CLI Modes** - Interactive (REPL) vs non-interactive (single prompt)
5. **User Feedback** - Visual indicators during agent processing
6. **HTTP Request/Response** - POST, headers, JSON serialization for API calls
7. **API Structure** - Request bodies, response parsing, error handling
8. **System Prompts** - Defining agent persona and behavior rules
9. **Message History** - Stateless API requires sending full conversation each turn
10. **Message Roles** - user/assistant alternation pattern
11. **Streaming/SSE** - Server-Sent Events for real-time token display
12. **Callback Pattern** - Process data chunks as they arrive
13. **Tool Use** - How LLMs request to use tools via structured responses
14. **Tool Definitions** - JSON Schema for tool inputs
15. **Tool Results** - Sending execution results back to continue conversation

---

## Code Architecture Notes
<!-- Document the evolving architecture as we build -->

```
Current structure:
src/
  main.rs       # Entry point (hello world)

Planned structure (will evolve):
src/
  main.rs       # Entry point, CLI setup
  agent/        # Core agent loop
  api/          # LLM API integration
  tools/        # Tool implementations
  config/       # Configuration management
```

---

## Blockers and Challenges
<!-- Note any issues we encountered and how we resolved them -->

*None yet*

---

## Ideas for Later
<!-- Capture ideas that come up but aren't immediately relevant -->

*None yet*

---

*Last updated: January 15, 2026 - Session 1 (hooks configured)*
