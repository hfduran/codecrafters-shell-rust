use crate::{
    commands::{
        args_wrapper::ArgsWrapper,
        command::{Command, InvokableCommand},
        command_type::{get_command_type},
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct TypeCommand {
    args_wrapper: ArgsWrapper,
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        // TODO: expand to print types of all arguments.
        // now it's ignoring everyone after the first
        match self.args_wrapper.get_args_vec().get(0) {
            None => ReplControl::Continue,
            Some(identifier) => ReplControl::Print(get_command_type(&identifier).to_string()),
        }
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            args_wrapper: input.args_wrapper.clone(),
        })
    }
}

impl InvokableCommand for TypeCommand {
    const IDENTIFIER: &'static str = "type";
}
