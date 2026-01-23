use crate::command::Command;

pub mod collection;
pub mod registry;


pub trait BuiltinCommand : Command {
    const PROGRAM: &'static str;
}