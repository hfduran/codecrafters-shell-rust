use crate::commands::{
    command::{Command, InvokableCommand},
    command_types::{
        echo_command::EchoCommand, exit_command::ExitCommand, not_found_command::NotFoundCommand,
    },
};

pub struct CommandRegistry;

impl CommandRegistry {
    pub fn evaluate(command: &str, arg: &str) -> Box<dyn Command> {
        match command {
            ExitCommand::COMMAND => Box::from(ExitCommand),
            EchoCommand::COMMAND => Box::from(EchoCommand::new(arg)),
            v => Box::from(NotFoundCommand::new(v)),
        }
    }
}
