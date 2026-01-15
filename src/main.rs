/// Johnathan Agent - An AI Agent CLI
///
/// Topic 1: Agent = loop of observe -> think -> act
/// Topic 2: The REPL pattern - Read, Eval, Print, Loop
/// Topic 3: CLI interface - args, feedback, modes
/// Topic 4: HTTP Requests and API Basics

mod api;

use clap::Parser;
use std::io::{self, Write};

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
        println!("[API key loaded]\n");
    }

    // Two modes: interactive (REPL) or non-interactive (single prompt)
    match cli.prompt {
        Some(prompt) => {
            // Non-interactive: run once and exit
            run_once(&prompt, &api_key, cli.verbose);
        }
        None => {
            // Interactive: enter the REPL
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

    let response = eval(prompt, api_key, verbose);
    println!("{}", response);
}

/// Interactive mode: the REPL
fn run_repl(api_key: &str, verbose: bool) {
    println!("Type 'quit' or 'exit' to stop.\n");

    loop {
        let input = match read_input() {
            Some(input) => input,
            None => continue,
        };

        if should_exit(&input) {
            println!("Goodbye!");
            break;
        }

        let response = eval(&input, api_key, verbose);
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

/// EVAL: Process input and generate response
/// Now includes a "thinking" indicator and API key (for Topic 4)
fn eval(input: &str, api_key: &str, verbose: bool) -> String {
    // Show thinking indicator
    print!("Thinking...");
    io::stdout().flush().ok();

    // Call the Claude API
    let result = api::send_message(api_key, input);

    // Clear the thinking indicator
    print!("\r            \r");
    io::stdout().flush().ok();

    match result {
        Ok(response) => {
            if verbose {
                println!("[API call successful]");
            }
            response
        }
        Err(e) => {
            if verbose {
                println!("[API error: {}]", e);
            }
            format!("Error: {}", e)
        }
    }
}
