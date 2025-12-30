use crate::{commands::{command::{Command, ConstructibleCommand, InvokableCommand}, command_input::CommandInput}, repl::repl_control::ReplControl};

pub struct ExitCommand;

impl ExitCommand {
    fn new(_: &CommandInput) -> Self {
        Self {}
    }
}

impl Command for ExitCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Exit
    }
}

impl ConstructibleCommand for ExitCommand {
    fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl InvokableCommand for ExitCommand {
    const IDENTIFIER: &'static str = "exit";
}