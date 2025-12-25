pub mod repl_control;

use crate::{
    commands::{command::Command, command_registry::CommandRegistry},
    repl::repl_control::ReplControl,
};
use std::io::{self, Write};

pub fn repl() {
    loop {
        print_prompt();
        let input = read_input();
        let command = evaluate_command(&input);
        let control = command.execute();
        match control {
            ReplControl::Print(v) => println!("{}", v),
            ReplControl::Exit => return,
            ReplControl::Continue => (),
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
    let (identifier, argument) = split_once_whitespace(command);
    let command = CommandRegistry::evaluate(identifier, argument);
    command
    // now: check if the command has argument, and set them!!
}

fn split_once_whitespace(s: &str) -> (&str, &str) {
    match s.split_once(char::is_whitespace) {
        Some((first, rest)) => (first, rest.trim_start()),
        None => (s, ""),
    }
}