use crate::{
    command::{Command, builtin_command::BuiltinCommand},
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct EchoCommand {
    args: Vec<String>,
}

impl EchoCommand {
    fn new(input: &ReplInput) -> Self {
        Self {
            args: input.args.clone(),
        }
    }
}

impl Command for EchoCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(String::from(self.args.join(" ")))
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl BuiltinCommand for EchoCommand {
    const PROGRAM: &'static str = "echo";
}
