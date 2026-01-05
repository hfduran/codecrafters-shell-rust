pub mod builtin_command;
pub mod command_factory;
pub mod command_type;
pub mod sys_command;
pub mod args_wrapper;
pub mod not_found_command;

use crate::repl::{repl_control::ReplControl, repl_input::ReplInput};

pub trait Command {
    fn execute(&self) -> ReplControl;
    fn new_box(input: &ReplInput) -> Box<dyn Command>
    where
        Self: Sized; // can only be called by concrete types
}