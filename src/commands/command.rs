use crate::repl::repl_control::ReplControl;

pub trait Command {
    fn execute(&mut self) -> ReplControl;
}

pub trait InvokableCommand {
    const COMMAND: &'static str;
}
