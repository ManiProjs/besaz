use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn run(commands: Vec<String>) -> Result<()> {
    for command in commands {
        println!("{} {}", "Running".green(), command);

        let status = Command::new("sh").arg("-c").arg(&command).status()?;

        if !status.success() {
            anyhow::bail!("Command failed: {}", command);
        }
    }

    Ok(())
}
