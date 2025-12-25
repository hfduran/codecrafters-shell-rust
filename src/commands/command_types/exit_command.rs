use crate::{commands::command::{Command, InvokableCommand}, repl::repl_control::ReplControl};

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Exit
    }
}

impl InvokableCommand for ExitCommand {
    const COMMAND: &'static str = "exit";
}