use crate::{
    commands::{
        command::{Command, ConstructibleCommand, InvokableCommand},
        command_registry::CommandRegistry,
    },
    repl::{repl_control::ReplControl, repl_input::ReplInput},
    sys_commands::get_sys_command_path,
};

pub struct TypeCommand {
    argument: String,
}

enum CommandType {
    ShellBuiltin(String),
    SysCommand { identifier: String, path: String },
    InvalidCommand(String),
}

impl CommandType {
    fn to_string(&self) -> String {
        match self {
            CommandType::ShellBuiltin(identifier) => format!("{} is a shell builtin", &identifier),
            CommandType::InvalidCommand(identifier) => format!("{} not found", &identifier),
            CommandType::SysCommand { identifier, path } => format!("{} is {}", identifier, path),
        }
    }
}

impl TypeCommand {
    fn get_type(&self) -> CommandType {
        let identifier = &self.argument;

        if CommandRegistry::global().is_registered(&identifier) {
            return CommandType::ShellBuiltin(identifier.clone());
        }

        if let Ok(sys_command) = get_sys_command_path(identifier) {
            return CommandType::SysCommand {
                identifier: identifier.clone(),
                path: sys_command,
            };
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
    fn new_box(input: &ReplInput) -> Box<dyn Command> {
        Box::from(TypeCommand {
            argument: input.clone_argument(),
        })
    }
}

impl InvokableCommand for TypeCommand {
    const IDENTIFIER: &'static str = "type";
}
