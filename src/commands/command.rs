use crate::repl::repl_control::ReplControl;

use super::command_input::CommandInput;

pub trait Command {
    fn execute(&self) -> ReplControl;
}

pub trait InvokableCommand {
    const IDENTIFIER: &'static str;
}

pub trait ConstructibleCommand: Command {
    fn new_box(input: &CommandInput) -> Box<dyn Command>;
}