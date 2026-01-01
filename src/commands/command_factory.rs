use crate::{
    commands::{
        builtin_commands::{
            builtin_commands_collection::not_found_command::NotFoundCommand,
            builtin_commands_registry::BuiltinCommandsRegistry,
        },
        command::Command,
        command_type::{CommandType, get_command_type},
        sys_commands::SysCommand,
    },
    repl::repl_input::ReplInput,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_command(input: &ReplInput) -> Box<dyn Command> {
        let command_type = get_command_type(&input.identifier);

        match command_type {
            CommandType::ShellBuiltin(_) => BuiltinCommandsRegistry::global()
                .get_constructor(input.identifier.as_str())
                .map(|constructor| constructor(input))
                .unwrap(),
            CommandType::SysCommand { .. } => Box::from(SysCommand::new_box(input)),
            CommandType::InvalidCommand(_) => Box::from(NotFoundCommand::new_box(input)),
        }
    }
}
