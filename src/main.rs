/// Johnathan Agent - An AI Agent CLI
///
/// Topic 1: Agent = loop of observe -> think -> act
/// Topic 2: The REPL pattern - Read, Eval, Print, Loop
/// Topic 3: CLI interface - args, feedback, modes
/// Topic 4: HTTP Requests and API Basics
/// Topic 5: The Anthropic API - system prompts, message history

mod api;

use api::Message;
use clap::Parser;
use std::io::{self, Write};

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

    if cli.verbose {
        println!("[verbose mode enabled]");
        println!("[API key loaded]");
        println!("[System prompt: {} chars]\n", SYSTEM_PROMPT.len());
    }

    // Two modes: interactive (REPL) or non-interactive (single prompt)
    match cli.prompt {
        Some(prompt) => {
            // Non-interactive: run once and exit
            run_once(&prompt, &api_key, cli.verbose);
        }
        None => {
            // Interactive: enter the REPL with conversation history
            run_repl(&api_key, cli.verbose);
        }
    }
}

/// Non-interactive mode: process a single prompt and exit
fn run_once(prompt: &str, api_key: &str, verbose: bool) {
    if verbose {
        println!("[non-interactive mode]");
        println!("[prompt: {}]\n", prompt);
    }

    // Single message, no history needed
    let messages = vec![Message::user(prompt)];
    let response = eval(messages, api_key, verbose);
    println!("{}", response);
}

/// Interactive mode: the REPL with conversation history
fn run_repl(api_key: &str, verbose: bool) {
    println!("Type 'quit' or 'exit' to stop.\n");

    // Conversation history persists across turns
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

        // Add user message to history
        history.push(Message::user(&input));

        if verbose {
            println!("[history: {} messages]", history.len());
        }

        // Get response with full history
        let response = eval(history.clone(), api_key, verbose);

        // Add assistant response to history
        history.push(Message::assistant(&response));

        println!("{}\n", response);
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

/// EVAL: Send messages to Claude and get response
fn eval(messages: Vec<Message>, api_key: &str, verbose: bool) -> String {
    print!("Thinking...");
    io::stdout().flush().ok();

    // Call API with history and system prompt
    let result = api::send_messages(api_key, messages, Some(SYSTEM_PROMPT));

    // Clear thinking indicator
    print!("\r            \r");
    io::stdout().flush().ok();

    match result {
        Ok(response) => {
            if verbose {
                println!("[stop_reason: {}]", response.stop_reason);
            }
            response.text
        }
        Err(e) => {
            if verbose {
                println!("[API error: {}]", e);
            }
            format!("Error: {}", e)
        }
    }
}
