//! REPL (Read-Eval-Print Loop)

use miette::{IntoDiagnostic, Result};
use std::io::{self, Write};

const PROMPT: &str = ">> ";
const CONT_PROMPT: &str = ".. ";

pub struct Repl {
    verbose: bool,
    history: Vec<String>,
}

impl Repl {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Synton REPL v{}", env!("CARGO_PKG_VERSION"));
        println!("Press Ctrl+C to exit\n");

        let mut input = String::new();
        let mut continued = false;

        loop {
            let prompt = if continued { CONT_PROMPT } else { PROMPT };
            print!("{}", prompt);
            io::stdout().flush().into_diagnostic()?;

            use std::io::BufRead;
            let mut line = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut line)
                .into_diagnostic()?;

            // Handle EOF
            if line.is_empty() {
                println!();
                break;
            }

            let line = line.trim();

            // Handle special commands
            match line {
                ":quit" | ":q" | ":exit" => break,
                ":clear" => {
                    input.clear();
                    continued = false;
                    continue;
                }
                ":history" => {
                    for (i, cmd) in self.history.iter().enumerate() {
                        println!("  {}: {}", i + 1, cmd);
                    }
                    continue;
                }
                _ => {}
            }

            input.push_str(line);
            input.push('\n');

            // Try to parse
            match synton_parser::Parser::new().parse_stmt(&input) {
                Ok(_stmt) => {
                    // Successfully parsed
                    self.history.push(input.clone());
                    input.clear();
                    continued = false;
                }
                Err(_) => {
                    // Maybe incomplete input
                    continued = true;
                }
            }
        }

        Ok(())
    }
}
