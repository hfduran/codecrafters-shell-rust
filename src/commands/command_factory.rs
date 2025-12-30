use std::collections::HashMap;

use crate::commands::{
    command::{Command, InvokableCommand},
    command_input::CommandInput,
    command_types::{
        echo_command::EchoCommand, exit_command::ExitCommand, not_found_command::NotFoundCommand, type_command::TypeCommand,
    },
};

pub type CommandConstructor = fn(&CommandInput) -> Box<dyn Command>;

pub struct CommandFactory {
    registry: HashMap<&'static str, CommandConstructor>,
}

impl CommandFactory {
    pub fn new() -> Self {
        let mut registry: HashMap<&'static str, CommandConstructor> = HashMap::new();

        registry.insert(ExitCommand::IDENTIFIER, |input| {
            ExitCommand::new_box(input)
        });
        registry.insert(EchoCommand::IDENTIFIER, |input| {
            EchoCommand::new_box(input)
        });
        registry.insert(TypeCommand::IDENTIFIER, |input| {
            TypeCommand::new_box(input)
        });

        Self { registry }
    }

    pub fn create_command(&self, input: &CommandInput) -> Box<dyn Command> {
        self.registry
            .get(input.identifier.as_str())
            .map(|constructor| constructor(input))
            .unwrap_or_else(|| NotFoundCommand::new_box(input))
    }

    pub fn is_identifier_in_registry(&self, identifier: &str) -> bool {
        self.registry.get(identifier).is_some()
    }
}
