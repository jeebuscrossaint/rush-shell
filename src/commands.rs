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
        .output()
        .expect("Failed to execute command");

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

