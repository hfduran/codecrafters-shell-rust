use std::{
    io::{self, Write},
    process,
};

fn main() {
    repl()
}

fn repl() {
    loop {
        print_prompt();
        let input = read_input();
        evaluate_command(&input);
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

fn evaluate_command(command: &str) {
    match command {
        "exit" => execute_exit_command(),
        _ => print_command_not_found(command),
    }
}

fn print_command_not_found(command: &str) {
    println!("{}: command not found", command)
}

fn execute_exit_command() {
    process::exit(0);
}
