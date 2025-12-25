use crate::{commands::command::Command, repl::repl_control::ReplControl};

pub struct NotFoundCommand {
    pub command: String,
}

impl NotFoundCommand {
    const NOT_FOUND_STRING: &'static str = ": command not found";
}

impl Command for NotFoundCommand {
    fn execute(&mut self) -> ReplControl {
        ReplControl::Print(String::from(format!(
            "{}{}",
            self.command,
            Self::NOT_FOUND_STRING
        )))
    }
}
