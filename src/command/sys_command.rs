use std::process::Command as RealCommand;

use anyhow::Result;
use which::which;

use crate::command::Command;

pub fn get_sys_command_path(identifier: &str) -> Result<String> {
    Ok(which(identifier)?.display().to_string())
}

pub struct SysCommand {
    pub program: String,
    pub args: Vec<String>,
}

impl Command for SysCommand {
    fn execute(&self) -> crate::repl::repl_control::ReplControl {
        let _ = RealCommand::new(&self.program)
            .args(&self.args)
            .spawn()
            .unwrap()
            .wait();
        crate::repl::repl_control::ReplControl::Continue
    }
    fn new_box(input: &crate::repl::repl_input::ReplInput) -> Box<dyn Command> {
        Box::from(Self {
            program: input.program.clone(),
            args: input.args.clone(),
        })
    }
}
