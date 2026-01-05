use crate::{
    command::{Command, builtin_command::BuiltinCommand},
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct ExitCommand;

impl ExitCommand {
    fn new(_: &ReplInput) -> Self {
        Self {}
    }
}

impl Command for ExitCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Exit
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl BuiltinCommand for ExitCommand {
    const IDENTIFIER: &'static str = "exit";
}
