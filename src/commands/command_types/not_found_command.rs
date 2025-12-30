use crate::{
    commands::{command::Command, command_input::CommandInput},
    repl::repl_control::ReplControl,
};

pub struct NotFoundCommand {
    pub command: String,
}

impl NotFoundCommand {
    const NOT_FOUND_STRING: &'static str = ": command not found";

    fn new(input: &CommandInput) -> Self {
        Self {
            command: input.clone_identifier(),
        }
    }

    pub fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl Command for NotFoundCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(format!("{}{}", self.command, Self::NOT_FOUND_STRING))
    }
}
