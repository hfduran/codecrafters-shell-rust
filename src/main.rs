use std::io::{self, Write};

fn main() {
    repl()
}

fn repl() {
    loop {
        print_prompt();
        let input = read_input();
        print_command_not_found(&input);
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn print_command_not_found(command: &str) {
    println!("{}: command not found", command)
}
