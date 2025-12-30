use crate::{
    commands::{command::{Command, InvokableCommand}, command_input::CommandInput},
    repl::repl_control::ReplControl,
};

pub struct EchoCommand {
    arg: String,
}

impl EchoCommand {
    fn new(input: &CommandInput) -> Self {
        Self {
            arg: input.clone_argument(),
        }
    }

    pub fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl InvokableCommand for EchoCommand {
    const IDENTIFIER: &'static str = "echo";
}

impl Command for EchoCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(String::from(&self.arg))
    }
}
