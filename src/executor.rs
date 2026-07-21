use anyhow::{Result, bail};
use std::collections::HashSet;

use crate::{
    config::{Config, Task},
    runner,
};

pub struct Executor<'a> {
    config: &'a Config,
    completed: HashSet<String>,
    visiting: HashSet<String>,
}

impl<'a> Executor<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            completed: HashSet::new(),
            visiting: HashSet::new(),
        }
    }

    pub fn execute(&mut self, name: &str) -> Result<()> {
        if self.completed.contains(name) {
            return Ok(());
        }

        if self.visiting.contains(name) {
            bail!("Circular dependency detected involving '{}'", name);
        }

        let task = self
            .config
            .tasks
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Task '{}' not found", name))?;

        self.visiting.insert(name.to_string());

        if let Some(dependencies) = &task.depends {
            for dependency in dependencies {
                self.execute(dependency)?;
            }
        }

        self.run_task(name, task)?;

        self.visiting.remove(name);
        self.completed.insert(name.to_string());

        Ok(())
    }

    fn run_task(&self, name: &str, task: &Task) -> Result<()> {
        println!("\n▶ {}", name);

        runner::run(task.run.commands(), task.env.as_ref())?;

        Ok(())
    }
}
