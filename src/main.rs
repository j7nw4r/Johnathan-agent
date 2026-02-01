/// Johnathan Agent - An AI Agent CLI
///
/// Topic 1: Agent = loop of observe -> think -> act
/// Topic 2: The REPL pattern - Read, Eval, Print, Loop
/// Topic 3: CLI interface - args, feedback, modes
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history
/// Topic 6: Streaming Responses - real-time token display
/// Topic 8: Tool Use / Function Calling
/// Topic 9: Designing a Tool System

mod api;
mod tools;

use api::Message;
use clap::Parser;
use std::io::{self, Write};
use tools::{GetTimeTool, ToolRegistry};

/// System prompt defines the agent's persona and behavior
const SYSTEM_PROMPT: &str = r#"You are Johnathan, an AI coding assistant.

You help users with programming tasks. Be concise and direct.
When asked to perform tasks, explain what you're doing briefly.

You are running as a CLI agent and can have multi-turn conversations."#;

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

fn main() {
    let cli = Cli::parse();

    println!("Johnathan Agent v0.1.0");
    println!("=======================\n");

    // Get API key from environment
    let api_key = match std::env::var("ANTHROPIC_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: ANTHROPIC_API_KEY environment variable not set");
            eprintln!("Set it with: export ANTHROPIC_API_KEY=your-key-here");
            std::process::exit(1);
        }
    };

    // Set up the tool registry
    let mut registry = ToolRegistry::new();
    registry.register(GetTimeTool::new());

    if cli.verbose {
        println!("[verbose mode enabled]");
        println!("[API key loaded]");
        println!("[System prompt: {} chars]", SYSTEM_PROMPT.len());
        println!("[tools registered: {}]\n", registry.definitions().len());
    }

    // Two modes: interactive (REPL) or non-interactive (single prompt)
    match cli.prompt {
        Some(prompt) => {
            run_once(&prompt, &api_key, &registry, cli.verbose);
        }
        None => {
            run_repl(&api_key, &registry, cli.verbose);
        }
    }
}

/// Non-interactive mode: process a single prompt and exit
fn run_once(prompt: &str, api_key: &str, registry: &ToolRegistry, verbose: bool) {
    if verbose {
        println!("[non-interactive mode]");
        println!("[prompt: {}]\n", prompt);
    }

    let messages = vec![Message::user(prompt)];
    let response = eval_streaming(messages, api_key, registry, verbose);
    // Response already printed via streaming, just add newline
    println!("\n{}", if verbose { format!("[done: {} chars]", response.len()) } else { String::new() });
}

/// Interactive mode: the REPL with conversation history
fn run_repl(api_key: &str, registry: &ToolRegistry, verbose: bool) {
    println!("Type 'quit' or 'exit' to stop.\n");

    let mut history: Vec<Message> = Vec::new();

    loop {
        let input = match read_input() {
            Some(input) => input,
            None => continue,
        };

        if should_exit(&input) {
            println!("Goodbye!");
            break;
        }

        history.push(Message::user(&input));

        if verbose {
            println!("[history: {} messages]", history.len());
        }

        // Get streaming response
        let response = eval_streaming(history.clone(), api_key, registry, verbose);

        // Add assistant response to history
        history.push(Message::assistant(&response));

        // Newline after streamed response
        println!("\n");
    }
}

fn read_input() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn should_exit(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower == "quit" || lower == "exit" || lower == "q"
}

/// EVAL with streaming: prints tokens as they arrive
fn eval_streaming(messages: Vec<Message>, api_key: &str, registry: &ToolRegistry, verbose: bool) -> String {
    // Show thinking indicator
    print!("Thinking...");
    io::stdout().flush().ok();

    let mut first_chunk = true;

    // Get tool definitions from registry
    let tools = registry.definitions();

    // Stream response, printing each chunk as it arrives
    let result = api::send_messages_streaming(
        api_key,
        messages,
        Some(SYSTEM_PROMPT),
        tools,
        |chunk| {
            // Clear "Thinking..." on first chunk
            if first_chunk {
                print!("\r            \r");
                io::stdout().flush().ok();
                first_chunk = false;
            }
            // Print chunk immediately (no newline)
            print!("{}", chunk);
            io::stdout().flush().ok();
        },
    );

    match result {
        Ok(response) => {
            if verbose {
                print!(" [stop: {}]", response.stop_reason);
            }
            response.text
        }
        Err(e) => {
            // Clear thinking indicator on error
            if first_chunk {
                print!("\r            \r");
            }
            let msg = format!("Error: {}", e);
            print!("{}", msg);
            msg
        }
    }
}
