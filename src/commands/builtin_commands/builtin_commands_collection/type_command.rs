use crate::{
    commands::{
        args_wrapper::ArgsWrapper,
        command::{Command, InvokableCommand},
        command_type::get_command_type,
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct TypeCommand {
    args_wrapper: ArgsWrapper,
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        let args_vec = self.args_wrapper.get_args_vec();
        let default_string = String::from("");

        let identifier = args_vec.get(0).unwrap_or(&default_string);
        ReplControl::Print(get_command_type(&identifier).to_string())
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
