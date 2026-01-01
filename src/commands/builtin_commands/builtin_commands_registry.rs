use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    commands::{
        builtin_commands::builtin_commands_collection::{
            echo_command::EchoCommand, exit_command::ExitCommand, type_command::TypeCommand,
        },
        command::{Command, InvokableCommand},
    },
    repl::repl_input::ReplInput,
};

pub type CommandConstructor = fn(&ReplInput) -> Box<dyn Command>;

pub struct BuiltinCommandsRegistry {
    commands: HashMap<&'static str, CommandConstructor>,
}

impl BuiltinCommandsRegistry {
    fn new() -> Self {
        let mut commands: HashMap<&'static str, CommandConstructor> = HashMap::new();

        // TODO: simplify with macros
        commands.insert(ExitCommand::IDENTIFIER, ExitCommand::new_box);
        commands.insert(EchoCommand::IDENTIFIER, EchoCommand::new_box);
        commands.insert(TypeCommand::IDENTIFIER, TypeCommand::new_box);

        Self { commands }
    }

    pub fn global() -> &'static BuiltinCommandsRegistry {
        static REGISTRY: Lazy<BuiltinCommandsRegistry> = Lazy::new(BuiltinCommandsRegistry::new);
        &REGISTRY
    }

    pub fn is_registered(&self, identifier: &str) -> bool {
        self.commands.contains_key(identifier)
    }

    pub fn get_constructor(&self, identifier: &str) -> Option<CommandConstructor> {
        self.commands.get(identifier).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_should_have_something() {
        let registry = BuiltinCommandsRegistry::global();
        assert!(registry.is_registered(ExitCommand::IDENTIFIER));
        assert!(registry.is_registered(EchoCommand::IDENTIFIER));
        assert!(registry.is_registered(TypeCommand::IDENTIFIER));
    }

    #[test]
    fn get_constructor_should_return_right_constructor() {
        let registry = BuiltinCommandsRegistry::global();
        let constructor = registry.get_constructor(ExitCommand::IDENTIFIER);
        assert!(constructor.is_some());

        let input = ReplInput {
            argument: String::from(""),
            identifier: String::from("exit"),
        };
        let _ = constructor.unwrap()(&input);
    }
}
