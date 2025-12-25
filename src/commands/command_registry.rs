use crate::commands::{
    command::{Command, InvokableCommand},
    command_types::{exit_command::ExitCommand, not_found_command::NotFoundCommand},
};

pub struct CommandRegistry;

impl CommandRegistry {
    pub fn evaluate(command: &str) -> Box<dyn Command> {
        match command {
            ExitCommand::COMMAND => Box::from(ExitCommand),
            v => Box::from(NotFoundCommand::new(v)),
        }
    }
}
