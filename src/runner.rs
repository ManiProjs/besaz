use anyhow::Result;
use colored::Colorize;
use std::{collections::HashMap, process::Command};

pub fn run(commands: Vec<String>, env: Option<&HashMap<String, String>>) -> Result<()> {
    for command in commands {
        println!("{} {}", "Running".green(), command);

        let mut process = Command::new("sh");

        process.arg("-c").arg(&command);

        if let Some(vars) = env {
            for (key, value) in vars {
                process.env(key, value);
            }
        }

        let status = process.status()?;

        if !status.success() {
            anyhow::bail!("Command failed: {}", command);
        }
    }

    Ok(())
}
