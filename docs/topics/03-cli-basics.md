# Topic 3: CLI Basics for Agents

## 1. Present

An agent CLI needs more than just a REPL. Users expect:
- **Pass a task directly**: `agent "fix the bug in main.rs"` (non-interactive)
- **Configuration flags**: `--model sonnet` or `--verbose`
- **Visual feedback**: Knowing when the agent is thinking vs acting
- **Graceful interruption**: Ctrl+C should stop cleanly

This isn't about making it pretty—it's about **communication between agent and user**.

---

## 2. Relate

Before this topic, the agent only had a bare REPL. You couldn't:
- Pass an initial prompt from the command line
- Know if it's thinking or waiting
- Configure anything without editing code

---

## 3. Explain

### Agent CLI Patterns

Most agent CLIs support two modes:

```
# Interactive mode (REPL)
$ agent
> do something

# Non-interactive mode (single task)
$ agent "do something"
```

Non-interactive is important for scripting and pipelines:
```bash
agent "summarize this file" < input.txt > summary.md
```

### Visual Feedback Matters

When an agent is making an API call that takes 5 seconds, silence feels broken. Users need to know:

| State | Feedback |
|-------|----------|
| Waiting for input | Prompt (`>`) |
| Calling LLM | Spinner or "Thinking..." |
| Running tool | "Running: read_file..." |
| Done | Response text |

### Configuration Hierarchy

Agent CLIs typically load config from multiple sources (in priority order):
1. Command-line flags (highest)
2. Environment variables
3. Project config file (`.agent.toml`)
4. User config file (`~/.config/agent/config.toml`)
5. Defaults (lowest)

### Environment Variables for Secrets

API keys should **never** be hardcoded or passed as CLI arguments (visible in process lists). Environment variables are the standard:

```bash
export ANTHROPIC_API_KEY=sk-...
agent "hello"
```

---

## 4. Implement

**Cargo.toml** - Added `clap` for argument parsing:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

**main.rs** - CLI structure:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "johnathan")]
#[command(about = "An AI agent CLI")]
struct Cli {
    /// Optional prompt to run (non-interactive mode)
    prompt: Option<String>,

    /// Print verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    // Get API key from environment
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: ANTHROPIC_API_KEY not set");
            std::process::exit(1);
        }
    };

    // Two modes based on whether prompt provided
    match cli.prompt {
        Some(prompt) => run_once(&prompt, &api_key, cli.verbose),
        None => run_repl(&api_key, cli.verbose),
    }
}
```

**Thinking indicator:**
```rust
fn eval(input: &str, api_key: &str, verbose: bool) -> String {
    print!("Thinking...");
    io::stdout().flush().ok();

    // Do work (API call)...

    print!("\r            \r");  // Clear with carriage return
    io::stdout().flush().ok();

    // Return response
}
```

---

## 5. Review

The CLI now supports:

```bash
$ johnathan                    # Interactive REPL
$ johnathan "do something"     # Non-interactive, single task
$ johnathan -v "do something"  # Verbose mode
```

**Structure:**

```
main()
├── Parse CLI args (clap)
├── Load API key from environment
└── match cli.prompt
    ├── Some → run_once()  # Process, print, exit
    └── None → run_repl()  # Loop until quit
```

**Agent concepts demonstrated:**

| Concept | Implementation |
|---------|----------------|
| Two modes | `Option<String>` prompt - Some = non-interactive, None = REPL |
| Feedback | "Thinking..." with carriage return to clear |
| Configuration | `-v` flag, environment variable for API key |
| Fail fast | Exit with clear error if API key missing |

---

## Key Takeaways

- Non-interactive mode enables automation (CI/CD, scripts)
- Visual feedback is critical when API calls take seconds
- Environment variables keep secrets out of command history
- Two modes: interactive for exploration, non-interactive for automation
