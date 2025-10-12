use std::process::Command;

use anyhow::{anyhow, Result};

/// Tries to run a command, and if it's not installed, returns an error.
pub fn run_optional_command(cmd: &str, args: &[&str], name: &str) -> Result<()> {
    match Command::new(cmd).args(args).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(anyhow!("`{} {}` failed", cmd, args.join(" "))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(anyhow!(
            "Optional tool '{}' is not installed. Please install it to use this feature.",
            name
        )),
        Err(e) => Err(anyhow!("Failed to run {}: {}", name, e)),
    }
}
