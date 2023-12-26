// Essential commands basically every shell should have, right?

pub fn echo(args: &[String]) {
    let output = args
        .iter()
        .map(|arg| arg.trim_matches('"'))
        .collect::<Vec<&str>>()
        .join(" ");
    println!("{}", output);
}

pub fn help(_args: &[String]) {
    println!("    help: display help");
    println!("    echo: echo input");
    println!("    exit: exit the shell");
    println!("    cd: change directory");
    println!("    pwd: print working directory");
    println!("    color: change text color");
    println!("    touch: create file");
    println!("    rm: remove file");
    println!("    ls: list files in directory");
}

pub fn exit(_args: &[String]) {
    std::process::exit(0);
}

use std::env;
use std::path::Path;

pub fn cd(args: &[String]) {
    if args.is_empty() {
        eprintln!("cd: expected argument");
        return;
    }

    let new_dir = &args[0];
    let path = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("cd: {}", e);
    }
}

pub fn pwd(_args: &[String]) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

use termion::color;

pub fn color(args: &[String]) {
    if args.is_empty() {
        eprintln!("color: expected a color code");
        return;
    }

    let color_code = &args[0];

    if color_code.starts_with('#') && color_code.len() == 7 {
        let r = u8::from_str_radix(&color_code[1..3], 16).unwrap();
        let g = u8::from_str_radix(&color_code[3..5], 16).unwrap();
        let b = u8::from_str_radix(&color_code[5..7], 16).unwrap();

        print!("{}", color::Fg(color::Rgb(r, g, b)));
    } else {
        eprintln!("color: invalid color code");
    }
}

use std::fs::OpenOptions;
use std::fs;

pub fn touch(args: &[String]) {
    if args.is_empty() {
        eprintln!("touch: expected filename");
        return;
    }

    let filename = &args[0];

    if let Err(e) = OpenOptions::new().create(true).write(true).open(filename) {
        eprintln!("touch: {}", e);
    }
}

pub fn rm(args: &[String]) {
    if args.is_empty() {
        eprintln!("rm: expected filename");
        return;
    }

    let filename = &args[0];

    if let Err(e) = fs::remove_file(filename) {
        eprintln!("rm: {}", e);
    }
}

pub fn ls(_args: &[String]) {
    let path = env::current_dir().unwrap();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        println!("{}", file_name);
                    }
                }
            }
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

use std::process::Command;

pub fn run_program(name: &str, args: &[String]) {
    let output = Command::new(name)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}
use rand::Rng;
use std::cmp::max;
use std::i32::MAX;

pub fn random(args: &[String]) {
    let mut rng = rand::thread_rng();
    let lower = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let upper = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(MAX);

    let number = rng.gen_range(lower..max(upper, lower + 1)); // Ensure the upper bound is always greater than the lower bound
    println!("{}", number);
}

use std::time::Instant;
use std::process::Command as ProcessCommand;

pub fn time(args: &[String]) {
    if args.is_empty() {
        eprintln!("time: expected command");
        return;
    }

    let start = Instant::now();

    let output = ProcessCommand::new(&args[0])
        .args(&args[1..])
        .output();

    match output {
        Ok(output) => {
            let duration = start.elapsed();

            println!("Command executed in: {:?}", duration);
            println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        },
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

use meval::eval_str;
use regex::Regex;

pub fn math(args: &[String]) {
    let expression = args.join(" ");
    let re = Regex::new(r"deg\((.*?)\)").unwrap();
    let expression = re.replace_all(&expression, |caps: &regex::Captures| {
        let deg_value: f64 = caps[1].parse().unwrap();
        let rad_value = deg(deg_value);
        rad_value.to_string()
    });

    let res = eval_str(&expression);

    match res {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn deg(n: f64) -> f64 {
    n * std::f64::consts::PI / 180.0
}