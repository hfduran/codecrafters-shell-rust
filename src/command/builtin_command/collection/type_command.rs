use crate::{
    command::{
        Command, builtin_command::BuiltinCommand,
        command_type::get_command_type,
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
};

pub struct TypeCommand {
    args: Vec<String>,
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        // TODO: expand to print types of all arguments.
        // now it's ignoring everyone after the first
        match self.args.get(0) {
            None => ReplControl::Continue,
            Some(program) => ReplControl::Print(get_command_type(&program).to_string()),
        }
    }
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            args: input.args.clone(),
        })
    }
}

impl BuiltinCommand for TypeCommand {
    const PROGRAM: &'static str = "type";
}
