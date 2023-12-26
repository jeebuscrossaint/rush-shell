use crate::parser::Command;
use crate::commands::{echo, help, exit, cd, pwd, color, touch, rm, ls, run_program, random, time, math};

pub fn execute(command: Command) {
    if command.name.is_empty() {
        return;
    }

    match command.name.as_str() {
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
        _ => run_program(&command.name, &command.args),
    }
}