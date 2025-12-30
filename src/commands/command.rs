use crate::repl::repl_control::ReplControl;

pub trait Command {
    fn execute(&self) -> ReplControl;
}

pub trait InvokableCommand {
    const IDENTIFIER: &'static str;
}