use crate::{
    command::Command,
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct NotFoundCommand {
    pub identifier: String,
}

impl NotFoundCommand {
    const NOT_FOUND_STRING: &'static str = ": command not found";

    fn new(input: &ReplInput) -> Self {
        Self {
            identifier: input.clone_identifier(),
        }
    }
}

impl Command for NotFoundCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(format!("{}{}", self.identifier, Self::NOT_FOUND_STRING))
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}
