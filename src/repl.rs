pub mod repl_control;

use std::{
    io::{self, Write},
    process,
};

use crate::{commands::{
    command::Command,
    command_types::{exit_command::ExitCommand, not_found_command::NotFoundCommand},
}, repl::repl_control::ReplControl};

pub fn repl() {
    loop {
        print_prompt();
        let input = read_input();
        let mut command = evaluate_command(&input);
        let control = command.execute();
        match control {
            ReplControl::Print(v) => println!("{}", v),
            ReplControl::Exit => return,
            ReplControl::Continue => ()
        }
    }
}

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn evaluate_command(command: &str) -> Box<dyn Command> {
    match command {
        "exit" => Box::new(ExitCommand),
        _ => Box::new(NotFoundCommand {
            command: String::from(command),
        }),
    }
}

pub fn print_command_not_found(command: &str) {
    println!("{}: command not found", command)
}

pub fn execute_exit_command() {
    process::exit(0);
}
