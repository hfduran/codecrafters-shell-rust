use crate::{
    command::{Command, CommandOutput, builtin_command::BuiltinCommand},
    repl::repl_input::ReplInput,
};

pub struct ExitCommand;

impl ExitCommand {
    fn new(_: &ReplInput) -> Self {
        Self {}
    }
}

impl Command for ExitCommand {
    fn execute(&self) -> CommandOutput {
        CommandOutput::new(None, true)
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(Self::new(input))
    }
}

impl BuiltinCommand for ExitCommand {
    const PROGRAM: &'static str = "exit";
}
