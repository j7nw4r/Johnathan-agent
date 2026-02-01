# Topic 3: CLI Basics in Rust

## The Concept

A command-line interface (CLI) is how users interact with our agent. Good CLI design means:

- Clear, intuitive commands
- Helpful error messages
- Support for common patterns (flags, arguments)
- Two modes: interactive and scripted

## Using Clap for Argument Parsing

We use the `clap` crate with its derive macros for declarative CLI definition:

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

## Code Implementation

### Defining the CLI Structure

```rust
use clap::Parser;

/// An AI agent that can perform tasks
#[derive(Parser)]
#[command(name = "johnathan")]
#[command(about = "An AI agent CLI", long_about = None)]
struct Cli {
    /// Optional prompt to run (non-interactive mode)
    prompt: Option<String>,

    /// Print verbose output
    #[arg(short, long)]
    verbose: bool,
}
```

This gives us:
- `johnathan` - starts interactive REPL
- `johnathan "do something"` - runs single prompt
- `johnathan -v` or `johnathan --verbose` - enables debug output

### Parsing and Using Arguments

```rust
fn main() {
    let cli = Cli::parse();  // Parses args, exits on error

    println!("Johnathan Agent v0.1.0");
    println!("=======================\n");

    // Get API key from environment
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: ANTHROPIC_API_KEY environment variable not set");
            std::process::exit(1);
        }
    };

    // Verbose mode shows debug info
    if cli.verbose {
        println!("[verbose mode enabled]");
        println!("[API key loaded]");
    }

    // Route to appropriate mode
    match cli.prompt {
        Some(prompt) => run_once(&prompt, &api_key, cli.verbose),
        None => run_repl(&api_key, cli.verbose),
    }
}
```

## Environment Variables

Sensitive data like API keys should come from environment variables, not command-line arguments (which appear in process lists):

```rust
let api_key = std::env::var("ANTHROPIC_API_KEY")
    .expect("ANTHROPIC_API_KEY must be set");
```

Usage:
```bash
export ANTHROPIC_API_KEY=sk-ant-...
johnathan
```

## Verbose Mode Pattern

A common pattern: pass a `verbose` flag through your call stack for conditional debug output:

```rust
fn run_repl(api_key: &str, verbose: bool) {
    // ...
    if verbose {
        println!("[history: {} messages]", history.len());
    }
    // ...
}

fn eval(messages: Vec<Message>, api_key: &str, verbose: bool) -> String {
    // ...
    if verbose {
        println!("[sending {} messages to API]", messages.len());
    }
    // ...
}
```

## Auto-Generated Help

Clap automatically generates help from your doc comments:

```
$ johnathan --help
An AI agent CLI

Usage: johnathan [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Optional prompt to run (non-interactive mode)

Options:
  -v, --verbose  Print verbose output
  -h, --help     Print help
```

## Key Takeaways

1. **Derive macros** - Clap's `#[derive(Parser)]` makes CLI definition declarative
2. **Doc comments become help** - `///` comments appear in `--help` output
3. **Environment for secrets** - Never put API keys in command-line arguments
4. **Verbose flag pattern** - Thread through call stack for debug output
5. **Two modes** - Optional positional arg distinguishes interactive vs one-shot

## What's Next

Topic 4 connects to the LLM API, turning our skeleton into a real agent that can think.
