use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    commands::{
        builtin_commands::{
            echo_command::EchoCommand, exit_command::ExitCommand, type_command::TypeCommand,
        },
        command::{Command, ConstructibleCommand, InvokableCommand},
    },
    repl::repl_input::ReplInput,
};

pub type CommandConstructor = fn(&ReplInput) -> Box<dyn Command>;

pub struct CommandRegistry {
    commands: HashMap<&'static str, CommandConstructor>,
}

impl CommandRegistry {
    fn new() -> Self {
        let mut commands: HashMap<&'static str, CommandConstructor> = HashMap::new();

        // TODO: simplify with macros
        commands.insert(ExitCommand::IDENTIFIER, ExitCommand::new_box);
        commands.insert(EchoCommand::IDENTIFIER, EchoCommand::new_box);
        commands.insert(TypeCommand::IDENTIFIER, TypeCommand::new_box);

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
