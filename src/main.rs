mod config;
mod executor;
mod init;
mod runner;

use anyhow::Result;
use colored::Colorize;
use init::init;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(|arg| arg == "--init").unwrap_or(false) {
        return init();
    }

    let config = config::load()?;

    let Some(task_name) = args.get(1) else {
        println!("{}", "Available tasks:".bold());

        let mut tasks: Vec<_> = config.tasks.iter().collect();
        tasks.sort_by_key(|(name, _)| *name);

        for (name, task) in tasks {
            match &task.desc {
                Some(desc) => {
                    println!("  {:<15} {}", name.green(), desc);
                }

                None => {
                    println!("  {}", name.green());
                }
            }
        }

        return Ok(());
    };

    let mut executor = executor::Executor::new(&config);

    executor.execute(task_name)?;

    Ok(())
}
