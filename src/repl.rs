pub mod repl_control;
pub mod repl_input;

use crate::{
    commands::{command::Command, command_factory::CommandFactory},
    repl::{repl_control::ReplControl, repl_input::ReplInput},
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
    let command_input = ReplInput::new(identifier, argument);
    CommandFactory::create_command(&command_input)
    // TODO: check if it's sys command and if yes, call it
}

fn split_once_whitespace(s: &str) -> (&str, &str) {
    match s.split_once(char::is_whitespace) {
        Some((first, rest)) => (first, rest.trim_start()),
        None => (s, ""),
    }
}