# Johnathan Agent

An AI agent CLI built from scratch while learning the underlying concepts.

## What This Is

An educational project that builds a Claude-powered coding agent step-by-step, covering:
- Agent loops and REPL patterns
- LLM API integration (HTTP, streaming, tool use)
- Context management and conversation history
- Tool implementation (file ops, bash, search)

## Structure

```
src/
  main.rs       # CLI and agent loop
  api/          # Claude API client
docs/           # Topic writeups for review
```

## Usage

```bash
export ANTHROPIC_API_KEY=your-key
cargo run                    # Interactive REPL
cargo run -- "your prompt"   # Single command
```

## Learning

Run `/teach` in Claude Code to enter Socratic teaching mode. See `TOPICS.md` for progress and `LEARNING_PLAN.md` for curriculum.
