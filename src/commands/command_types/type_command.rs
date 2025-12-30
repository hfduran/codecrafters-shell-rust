use crate::{
    commands::{
        command::{Command, ConstructibleCommand, InvokableCommand},
        command_factory::CommandRegistry,
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
        if CommandRegistry::global().is_registered(&self.argument) {
            return CommandType::ShellBuiltin(self.argument.clone());
        }
        CommandType::InvalidCommand(self.argument.clone())
    }
}

impl Command for TypeCommand {
    fn execute(&self) -> ReplControl {
        ReplControl::Print(self.get_type().to_string())
    }
}

impl ConstructibleCommand for TypeCommand {
    fn new_box(input: &CommandInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            argument: input.clone_argument(),
        })
    }
}

impl InvokableCommand for TypeCommand {
    const IDENTIFIER: &'static str = "type";
}
