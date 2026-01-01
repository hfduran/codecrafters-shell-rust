use crate::{
    commands::{
        command::{Command, InvokableCommand},
        command_type::get_command_type,
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct TypeCommand {
    argument: String,
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(get_command_type(&self.argument).to_string())
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            argument: input.clone_argument_as_str(),
        })
    }
}

impl InvokableCommand for TypeCommand {
    const IDENTIFIER: &'static str = "type";
}
