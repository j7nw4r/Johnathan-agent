/// Johnathan Agent - An AI Agent CLI
///
/// Topic 1: Agent = loop of observe -> think -> act
/// Topic 2: The REPL pattern - Read, Eval, Print, Loop

use std::io::{self, Write};

fn main() {
    println!("Johnathan Agent v0.1.0");
    println!("=======================");
    println!("Type 'quit' or 'exit' to stop.\n");

    // THE AGENT LOOP (REPL Pattern)
    // This runs until the user decides to quit
    loop {
        // 1. READ - Get input from the user
        let input = match read_input() {
            Some(input) => input,
            None => continue, // Empty input, prompt again
        };

        // Check for exit commands
        if should_exit(&input) {
            println!("Goodbye!");
            break;
        }

        // 2. EVAL - Process the input (later: send to LLM)
        let response = eval(&input);

        // 3. PRINT - Show the result
        println!("{}\n", response);

        // 4. LOOP - automatically continues
    }
}

/// READ: Prompt the user and get their input
/// Returns None if input is empty or there's an error
fn read_input() -> Option<String> {
    // Print prompt and flush to ensure it appears before input
    print!("> ");
    io::stdout().flush().ok()?;

    // Read a line from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    // Trim whitespace and check if empty
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Check if the user wants to exit
fn should_exit(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower == "quit" || lower == "exit" || lower == "q"
}

/// EVAL: Process the input and generate a response
/// For now, this is a simple echo. Later: call LLM, handle tool use
fn eval(input: &str) -> String {
    // This is where the magic will happen.
    // Future structure:
    //
    // 1. Add user message to conversation history
    // 2. Send history to LLM
    // 3. Get response
    // 4. If response contains tool call:
    //    a. Execute tool
    //    b. Add tool result to history
    //    c. Go back to step 2 (inner loop!)
    // 5. Return final text response

    // For now, just acknowledge the input
    format!("[Echo] You said: {}", input)
}
