use crate::parser::parse;
use crate::executor::execute;
use crate::config::{load_config};
use std::io::{self, Write};
use rustyline::completion::{Completer, Pair};
use rustyline::Context;
mod parser;
mod executor;
mod commands;
mod config;

struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        _line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let completions = vec![
            Pair {
                display: "echo".into(),
                replacement: "echo".into(),
            },
            Pair {
                display: "help".into(),
                replacement: "help".into(),
            },
            Pair {
                display: "exit".into(),
                replacement: "exit".into(),
            },
            Pair {
                display: "cd".into(),
                replacement: "cd".into(),
            },
            Pair {
                display: "pwd".into(),
                replacement: "pwd".into(),
            },
            Pair {
                display: "color".into(),
                replacement: "color".into(),
            },
            Pair {
                display: "touch".into(),
                replacement: "touch".into(),
            },
            Pair {
                display: "rm".into(),
                replacement: "rm".into(),
            },
            Pair {
                display: "ls".into(),
                replacement: "ls".into(),
            },
        ];

        Ok((pos, completions))
    
    }
}

fn main() {
    let config = load_config().unwrap_or_else(|err| {
        eprintln!("Error loading config: {}", err);
        std::process::exit(1);
    });

    for cmd in config.init_commands {
        let command = parse(&cmd);
        execute(command);
    }

    loop {
        print!("$ "); 
        io::stdout().flush().unwrap(); 

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = parse(&input);
        execute(command);
    }
}