use anyhow::{Result};
use which::which;

pub fn get_sys_command_path(identifier: &str) -> Result<String> {
    Ok(which(identifier)?.display().to_string())
}