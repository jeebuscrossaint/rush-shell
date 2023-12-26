use crate::parser::parse;
use crate::executor::execute;
use crate::config::{load_config};
mod parser;
mod executor;
mod commands;
mod config;


use rustyline::Editor;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Result;
use rustyline::Helper;

struct Completions;

impl Completer for Completions {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let mut completions = Vec::new();

        if line.starts_with("ti") {
            completions.push(Pair {
                display: "time".into(),
                replacement: "time".into(),
            });
        } else if line.starts_with("ex") {
            completions.push(Pair {
                display: "exit".into(),
                replacement: "exit".into(),
            });
        }

        Ok((pos, completions))
    }
}

impl Highlighter for Completions {}

impl Hinter for Completions {
    type Hint = String;
}
impl Validator for Completions {}

impl Helper for Completions {}


fn main() {
    let config = load_config().unwrap_or_else(|err| {
        eprintln!("Error loading config: {}", err);
        std::process::exit(1);
    });

    for cmd in config.init_commands {
        let command = parse(&cmd);
        execute(command);
    }

    let mut rl = Editor::<Completions>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("❯ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let command = parse(&line);
                execute(command);
            },
            Err(_) => break,
        }
    }

    rl.save_history("history.txt").unwrap();
}