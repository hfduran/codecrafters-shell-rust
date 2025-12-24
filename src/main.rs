use std::io::{self, Write};

fn main() {
    print_prompt();
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
    read_input();
}

fn read_input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    print_command_not_found(&input);
}

fn print_command_not_found(command: &str) {
    print!("{}: command not found", command)
}
