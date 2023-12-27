use crate::parser::Command;
use crate::commands::{echo, help, exit, cd, pwd, color, touch, rm, ls, run_program, random, time, math};

pub fn execute(command: Command, aliases: &mut std::collections::HashMap<String, String>) {
    if command.name.is_empty() {
        return;
    }

    let name = match aliases.get(&command.name) {
        Some(alias) => alias.clone(),
        None => command.name,
    };

    match name.as_str() {
        "alias" => {
            if command.args.len() != 2 {
                eprintln!("alias: expected 2 arguments, got {}", command.args.len());
                return;
            }

            aliases.insert(command.args[0].clone(), command.args[1].clone());
        },
        "echo" => echo(&command.args),
        "help" => help(&command.args),
        "exit" => exit(&command.args),
        "cd" => cd(&command.args),
        "pwd" => pwd(&command.args),
        "color" => color(&command.args),
        "touch" => touch(&command.args),
        "rm" => rm(&command.args),
        "ls" => ls(&command.args),
        "random" => random(&command.args),
        "time" => time(&command.args),
        "math" => math(&command.args),
        _ => run_program(&name, &command.args),
    }
}