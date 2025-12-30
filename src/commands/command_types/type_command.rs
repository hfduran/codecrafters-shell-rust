use crate::{
    commands::{
        command::{Command, InvokableCommand},
        command_factory::CommandFactory,
        command_input::CommandInput,
    },
    repl::repl_control::ReplControl,
};

pub struct TypeCommand {
    argument: String,
}

enum CommandType {
    ShellBuiltin(String),
    InvalidCommand(String),
}

impl CommandType {
    fn to_string(&self) -> String {
        match self {
            CommandType::ShellBuiltin(identifier) => format!("{} is a shell builtin", &identifier),
            CommandType::InvalidCommand(identifier) => format!("{} not found", &identifier),
        }
    }
}

impl TypeCommand {
    fn get_type(&self) -> CommandType {
        let command_factory = CommandFactory::new();
        if command_factory.is_identifier_in_registry(&self.argument) {
            return CommandType::ShellBuiltin(self.argument.clone());
        }
        CommandType::InvalidCommand(self.argument.clone())
    }

    pub fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            argument: input.clone_argument(),
        })
    }
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(self.get_type().to_string())
    }
}

impl InvokableCommand for TypeCommand {
    const IDENTIFIER: &'static str = "type";
}
