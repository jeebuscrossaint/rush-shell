pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Command {
    let mut parts = input.trim().split_whitespace();
    let name = parts.next().unwrap_or("").to_string();
    let args = parts.map(|s| s.to_string()).collect();

    Command { name, args }
}