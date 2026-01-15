/// Johnathan Agent - An AI Agent CLI
///
/// Topic 1: Agent = loop of observe -> think -> act
/// Topic 2: The REPL pattern - Read, Eval, Print, Loop
/// Topic 3: CLI interface - args, feedback, modes

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

    if cli.verbose {
        println!("[verbose mode enabled]\n");
    }

    // Two modes: interactive (REPL) or non-interactive (single prompt)
    match cli.prompt {
        Some(prompt) => {
            // Non-interactive: run once and exit
            run_once(&prompt, cli.verbose);
        }
        None => {
            // Interactive: enter the REPL
            run_repl(cli.verbose);
        }
    }
}

/// Non-interactive mode: process a single prompt and exit
fn run_once(prompt: &str, verbose: bool) {
    if verbose {
        println!("[non-interactive mode]");
        println!("[prompt: {}]\n", prompt);
    }

    let response = eval(prompt, verbose);
    println!("{}", response);
}

/// Interactive mode: the REPL
fn run_repl(verbose: bool) {
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

        let response = eval(&input, verbose);
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
/// Now includes a "thinking" indicator
fn eval(input: &str, verbose: bool) -> String {
    // Show thinking indicator (will be replaced with real LLM call)
    print!("Thinking...");
    io::stdout().flush().ok();

    // Simulate work (later: actual API call)
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Clear the thinking indicator
    print!("\r            \r");
    io::stdout().flush().ok();

    if verbose {
        println!("[processed: {}]", input);
    }

    format!("[Echo] {}", input)
}
