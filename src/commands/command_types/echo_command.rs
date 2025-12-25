use crate::{
    commands::command::{Command, InvokableCommand},
    repl::repl_control::ReplControl,
};

pub struct EchoCommand {
    arg: String,
}

impl EchoCommand {
    pub fn new(arg: &str) -> Self {
        Self {
            arg: String::from(arg)
        }
    }
}

impl InvokableCommand for EchoCommand {
    const COMMAND: &'static str = "echo";
}

impl Command for EchoCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(String::from(&self.arg))
    }
}
