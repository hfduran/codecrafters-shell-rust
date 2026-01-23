use crate::{
    command::{
        Command,
        builtin_command::registry::BuiltinCommandsRegistry,
        command_type::{CommandType, get_command_type},
        not_found_command::NotFoundCommand,
        sys_command::SysCommand,
    },
    repl::repl_input::ReplInput,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_command(input: &ReplInput) -> Box<dyn Command> {
        let command_type = get_command_type(&input.program);

        match command_type {
            CommandType::ShellBuiltin(_) => BuiltinCommandsRegistry::global()
                .get_constructor(&input.program.as_str())
                .map(|constructor| constructor(input))
                .unwrap(), // kinda safe bc command_type already evaluated
            CommandType::SysCommand { .. } => Box::from(SysCommand::new_box(input)),
            CommandType::InvalidCommand(_) => Box::from(NotFoundCommand::new_box(input)),
        }
    }
}
