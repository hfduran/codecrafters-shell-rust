use std::process::Command as RealCommand;

use anyhow::Result;
use which::which;

use crate::commands::command::Command;

pub fn get_sys_command_path(identifier: &str) -> Result<String> {
    Ok(which(identifier)?.display().to_string())
}

pub struct SysCommand {
    pub path: String,
    pub args: Vec<String>,
}

impl Command for SysCommand {
    fn execute(&self) -> crate::repl::repl_control::ReplControl {
        let _ = RealCommand::new(&self.path)
            .args(&self.args)
            .spawn()
            .unwrap()
            .wait();
        crate::repl::repl_control::ReplControl::Continue
    }
}
