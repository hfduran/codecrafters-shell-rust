use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::commands::{
    command::{Command, ConstructibleCommand, InvokableCommand},
    command_input::CommandInput,
    command_types::{
        echo_command::EchoCommand, exit_command::ExitCommand, not_found_command::NotFoundCommand, type_command::TypeCommand,
    },
};

pub type CommandConstructor = fn(&CommandInput) -> Box<dyn Command>;

pub struct CommandRegistry {
    commands: HashMap<&'static str, CommandConstructor>,
}

impl CommandRegistry {
    fn new() -> Self {
        let mut commands: HashMap<&'static str, CommandConstructor> = HashMap::new();

        // TODO: simplify with macros
        commands.insert(ExitCommand::IDENTIFIER, |input| {
            ExitCommand::new_box(input)
        });
        commands.insert(EchoCommand::IDENTIFIER, |input| {
            EchoCommand::new_box(input)
        });
        commands.insert(TypeCommand::IDENTIFIER, |input| {
            TypeCommand::new_box(input)
        });

        Self { commands }
    }

    pub fn global() -> &'static CommandRegistry {
        static REGISTRY: Lazy<CommandRegistry> = Lazy::new(CommandRegistry::new);
        &REGISTRY
    }

    pub fn is_registered(&self, identifier: &str) -> bool {
        self.commands.contains_key(identifier)
    }

    pub fn get_constructor(&self, identifier: &str) -> Option<CommandConstructor> {
        self.commands.get(identifier).copied()
    }
}

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_command(input: &CommandInput) -> Box<dyn Command> {
        CommandRegistry::global()
            .get_constructor(input.identifier.as_str())
            .map(|constructor| constructor(input))
            .unwrap_or_else(|| NotFoundCommand::new_box(input))
    }
}
