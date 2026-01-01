use crate::{
    commands::command::{Command, InvokableCommand},
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct EchoCommand {
    arg: String,
}

impl EchoCommand {
    fn new(input: &ReplInput) -> Self {
        Self {
            arg: input.clone_argument_as_str(),
        }
    }
}

impl Command for EchoCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(String::from(&self.arg))
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl InvokableCommand for EchoCommand {
    const IDENTIFIER: &'static str = "echo";
}
