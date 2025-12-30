use crate::{commands::{command::{Command, InvokableCommand}, command_input::CommandInput}, repl::repl_control::ReplControl};

pub struct ExitCommand;

impl ExitCommand {
    fn new(_: &CommandInput) -> Self {
        Self {}
    }

    pub fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl Command for ExitCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Exit
    }
}

impl InvokableCommand for ExitCommand {
    const IDENTIFIER: &'static str = "exit";
}