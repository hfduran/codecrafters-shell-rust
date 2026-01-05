use std::process::Command as RealCommand;

use anyhow::Result;
use which::which;

use crate::command::{args_wrapper::ArgsWrapper, Command};

pub fn get_sys_command_path(identifier: &str) -> Result<String> {
    Ok(which(identifier)?.display().to_string())
}

pub struct SysCommand {
    pub identifier: String,
    pub args_wrapper: ArgsWrapper,
}

impl Command for SysCommand {
    fn execute(&self) -> crate::repl::repl_control::ReplControl {
        let _ = RealCommand::new(&self.identifier)
            .args(&self.args_wrapper.get_args_vec())
            .spawn()
            .unwrap()
            .wait();
        crate::repl::repl_control::ReplControl::Continue
    }
    fn new_box(input: &crate::repl::repl_input::ReplInput) -> Box<dyn Command> {
        Box::from(Self {
            args_wrapper: input.args_wrapper.clone(),
            identifier: input.clone_identifier(),
        })
    }
}
