use crate::{
    commands::{
        builtin_commands::{
            builtin_commands_collection::not_found_command::NotFoundCommand,
            builtin_commands_registry::CommandRegistry,
        },
        command::{Command, ConstructibleCommand},
    },
    repl::repl_input::ReplInput,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_command(input: &ReplInput) -> Box<dyn Command> {
        CommandRegistry::global()
            .get_constructor(input.identifier.as_str())
            .map(|constructor| constructor(input))
            .unwrap_or_else(|| NotFoundCommand::new_box(input))
    }
}
