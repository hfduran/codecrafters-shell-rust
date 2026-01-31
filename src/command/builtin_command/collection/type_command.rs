use crate::{
    command::{
        Command, CommandOutput, builtin_command::BuiltinCommand,
        command_type::get_command_type,
    },
    repl::repl_input::ReplInput,
};

pub struct TypeCommand {
    args: Vec<String>,
}

impl Command for TypeCommand {
    fn execute(&self) -> CommandOutput {
        // TODO: expand to print types of all arguments.
        // now it's ignoring everyone after the first
        let output = match self.args.get(0) {
            None => None,
            Some(program) => Some(get_command_type(&program).to_string()),
        };
        CommandOutput::from_output(output)
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
