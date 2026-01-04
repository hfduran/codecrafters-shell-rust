use crate::{
    commands::{
        args_wrapper::ArgsWrapper,
        command::{Command, InvokableCommand},
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct EchoCommand {
    args: ArgsWrapper,
}

impl EchoCommand {
    fn new(input: &ReplInput) -> Self {
        Self {
            args: input.args_wrapper.clone(),
        }
    }
}

impl Command for EchoCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(String::from(&self.args.get_args_vec().join(" ")))
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl InvokableCommand for EchoCommand {
    const IDENTIFIER: &'static str = "echo";
}
