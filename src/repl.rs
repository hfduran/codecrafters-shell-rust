pub mod repl_control;
pub mod repl_input;

use crate::{
    command::{command_factory::CommandFactory},
    repl::{repl_control::ReplControl, repl_input::ReplInput}, input_processing::extract_input_parts,
};
use std::io::{self, Write};

pub fn repl() {
    loop {
        print_prompt();
        let input = read_input();
        let control = evaluate_input(&input);
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

pub fn evaluate_input(input_str: &str) -> ReplControl {
    let parts = extract_input_parts(input_str);
    if let Some((program, argument)) = parts.split_first() {
        let repl_input = ReplInput::new(program, argument);
        let command = CommandFactory::create_command(&repl_input);
        command.execute()
    } else {
        ReplControl::Continue
    }
}