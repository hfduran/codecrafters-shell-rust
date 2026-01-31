pub mod builtin_command;
pub mod command_factory;
pub mod command_type;
pub mod not_found_command;
pub mod sys_command;

use crate::repl::repl_input::ReplInput;

pub struct CommandOutput {
    pub output: Option<String>,
    pub exit: bool,
}

impl CommandOutput {
    pub fn new(output: Option<String>, exit: bool) -> Self {
        Self { output, exit }
    }

    pub fn from_output(output: Option<String>) -> Self {
        Self { output, exit: false }
    }
}

pub trait Command {
    fn execute(&self) -> CommandOutput;
    fn new_box(input: &ReplInput) -> Box<dyn Command>
    where
        Self: Sized; // can only be called by concrete types
}
