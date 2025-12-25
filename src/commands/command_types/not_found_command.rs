use crate::{commands::command::Command, repl::repl_control::ReplControl};

pub struct NotFoundCommand {
    pub command: String,
}

impl NotFoundCommand {
    const NOT_FOUND_STRING: &'static str = ": command not found";

    pub fn new(command: &str) -> Self {
        Self {
            command: String::from(command),
        }
    }
}

impl Command for NotFoundCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(format!(
            "{}{}",
            self.command,
            Self::NOT_FOUND_STRING
        ))
    }
}
