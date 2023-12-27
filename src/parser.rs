pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();
    let name = parts.next().unwrap_or("").to_string();

    let mut args = parts.map(|s| s.to_string()).collect::<Vec<String>>();

    if name == "alias" && args.len() >= 1 {
        let rest_of_line = args.join(" ");
        let mut parts = rest_of_line.splitn(2, '=');
        args = parts.map(|s| s.trim().to_string()).collect::<Vec<String>>();
    }

    Command { name, args }
}