use crate::repl::{repl_control::ReplControl, repl_input::ReplInput};

pub trait Command {
    fn execute(&self) -> ReplControl;
    fn new_box(input: &ReplInput) -> Box<dyn Command>
    where
        Self: Sized; // can only be called by concrete types
}

pub trait InvokableCommand {
    const IDENTIFIER: &'static str;
}
