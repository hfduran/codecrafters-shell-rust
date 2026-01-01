use crate::{
    commands::builtin_commands::builtin_commands_registry::BuiltinCommandsRegistry,
    commands::sys_commands::get_sys_command_path,
};

pub enum CommandType {
    ShellBuiltin(String),
    SysCommand { identifier: String, path: String },
    InvalidCommand(String),
}

impl CommandType {
    pub fn to_string(&self) -> String {
        match self {
            CommandType::ShellBuiltin(identifier) => format!("{} is a shell builtin", &identifier),
            CommandType::InvalidCommand(identifier) => format!("{} not found", &identifier),
            CommandType::SysCommand { identifier, path } => format!("{} is {}", identifier, path),
        }
    }
}

pub fn get_command_type(identifier: &str) -> CommandType {
    if BuiltinCommandsRegistry::global().is_registered(&identifier) {
        return CommandType::ShellBuiltin(String::from(identifier));
    }

    if let Ok(sys_command) = get_sys_command_path(identifier) {
        return CommandType::SysCommand {
            identifier: String::from(identifier),
            path: sys_command,
        };
    }

    CommandType::InvalidCommand(String::from(identifier))
}
