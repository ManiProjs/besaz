use anyhow::Result;
use std::{fs, path::Path};

use colored::Colorize;

pub fn init() -> Result<()> {
    let path = Path::new("Besazfile");

    if path.exists() {
        println!("{}", "Besazfile already exists".yellow());
        return Ok(());
    }

    let content = r#"# Welcome to Besaz! 🚀
#
# Besaz lets you define your own project workflows.
#
# Create tasks here and run them with:
#
#   besaz <task>
#
# The only required field is:
#
#   run = "command to execute"
#
# Optional fields:
#
#   description -> Text shown when listing tasks
#   depends     -> Tasks that run before this task
#   env         -> Environment variables for the task


# A simple task
[tasks.build]

# Optional: shown in `besaz`
description = "Build the project"

# Required: command(s) to execute
run = "echo Add your build command here"


# Another simple task
[tasks.test]
description = "Run tests"
run = "echo Add your test command here"


# Tasks can have dependencies.
# Dependencies run before this task.
#
# [tasks.release]
# description = "Create a release"
# depends = ["test", "build"]
# run = "echo Package release"


# Tasks can run multiple commands.
#
# [tasks.ci]
# description = "Run CI checks"
#
# run = [
#     "echo Formatting",
#     "echo Testing",
#     "echo Building"
# ]


# Environment variables are optional.
#
# [tasks.deploy]
# description = "Deploy application"
#
# env = {
#     ENVIRONMENT = "production"
# }
#
# run = "./deploy.sh"
"#;

    fs::write(path, content)?;

    println!("{}", "✓ Created Besazfile".green());

    Ok(())
}
