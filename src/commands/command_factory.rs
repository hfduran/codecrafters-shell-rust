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
            CommandType::SysCommand { path, identifier } => Box::from(SysCommand {
                identifier: identifier,
                path: path,
                args: input.clone_argument_as_vec(),
            }),
            CommandType::InvalidCommand(identifier) => Box::from(NotFoundCommand {
                command: identifier,
            }),
        }
    }
}
