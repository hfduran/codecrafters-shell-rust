use crate::repl::{repl_control::ReplControl, repl_input::ReplInput};

pub trait Command {
    fn execute(&self) -> ReplControl;
}

pub trait InvokableCommand {
    const IDENTIFIER: &'static str;
}

pub trait ConstructibleCommand: Command {
    fn new_box(input: &ReplInput) -> Box<dyn Command>;
}