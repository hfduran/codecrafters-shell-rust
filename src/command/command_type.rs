use crate::{
    command::builtin_command::registry::BuiltinCommandsRegistry,
    command::sys_command::get_sys_command_path,
};

pub enum CommandType {
    ShellBuiltin(String),
    SysCommand { program: String, path: String },
    InvalidCommand(String),
}

impl CommandType {
    pub fn to_string(&self) -> String {
        match self {
            CommandType::ShellBuiltin(program) => format!("{} is a shell builtin", &program),
            CommandType::InvalidCommand(program) => format!("{} not found", &program),
            CommandType::SysCommand { program, path } => format!("{} is {}", program, path),
        }
    }
}

pub fn get_command_type(program: &str) -> CommandType {
    if BuiltinCommandsRegistry::global().is_registered(&program) {
        return CommandType::ShellBuiltin(String::from(program));
    }

    if let Ok(sys_command) = get_sys_command_path(program) {
        return CommandType::SysCommand {
            program: String::from(program),
            path: sys_command,
        };
    }

    CommandType::InvalidCommand(String::from(program))
}
