mod config;
mod runner;

use anyhow::Result;
use colored::Colorize;
use std::env;

fn main() -> Result<()> {
    let config = config::load()?;

    let args: Vec<String> = env::args().collect();

    let Some(task_name) = args.get(1) else {
        println!("{}", "Available tasks:".bold());

        for (name, task) in &config.tasks {
            match &task.desc {
                Some(desc) => println!("  {} - {}", name.green(), desc),
                None => println!("  {}", name.green()),
            }
        }

        return Ok(());
    };

    let task = config.tasks.get(task_name).ok_or_else(|| {
        anyhow::anyhow!(
            "Task '{}' does not exist.\n\
                 Run `besaz` to see available tasks.",
            task_name
        )
    })?;

    runner::run(task.run.commands())?;

    Ok(())
}
