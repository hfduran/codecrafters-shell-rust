pub mod repl_control;
pub mod repl_input;

use crate::{
    command::{Command, command_factory::CommandFactory},
    input_processing::{InputPart, parse_input},
    repl::{repl_control::ReplControl, repl_input::ReplInput},
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
    let parsed_input = parse_input(input_str);

    let mut word_sequence: Vec<String> = Vec::new();
    for part in parsed_input.get_parts() {
        match part {
            InputPart::Word(word) => {
                word_sequence.push(word);
            }
            InputPart::Operator(_) => {}
        }
    }

    if let Some((program, argument)) = word_sequence.split_first() {
        let repl_input = ReplInput::new(program, argument);
        let command = CommandFactory::create_command(&repl_input);
        execute_command(command)
    } else {
        ReplControl::Continue
    }
}

pub fn execute_command(command: Box<dyn Command>) -> ReplControl {
    let response = command.execute();
    if response.exit {
        return ReplControl::Exit;
    }
    if let Some(string) = response.output {
        return ReplControl::Print(string);
    }
    ReplControl::Continue
}
