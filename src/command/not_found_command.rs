use crate::{command::{Command, CommandOutput}, repl::repl_input::ReplInput};

pub struct NotFoundCommand {
    pub program: String,
}

impl NotFoundCommand {
    const NOT_FOUND_STRING: &'static str = ": command not found";

    fn new(input: &ReplInput) -> Self {
        Self {
            program: input.program.clone(),
        }
    }
}

impl Command for NotFoundCommand {
    fn execute(&self) -> CommandOutput {
        CommandOutput::from_output(Some(format!("{}{}", self.program, Self::NOT_FOUND_STRING)))
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}
