use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub depends: Option<Vec<String>>,
    pub desc: Option<String>,
    pub run: CommandList,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CommandList {
    Single(String),
    Multiple(Vec<String>),
}

impl CommandList {
    pub fn commands(&self) -> Vec<String> {
        match self {
            CommandList::Single(command) => {
                vec![command.clone()]
            }

            CommandList::Multiple(commands) => commands.clone(),
        }
    }
}

pub fn load() -> Result<Config> {
    let content = fs::read_to_string("Besazfile")
        .with_context(|| "Could not find Besazfile.\nRun `besaz init` to create one.")?;

    let config: Config = toml::from_str(&content)
        .with_context(|| "Failed to parse Besazfile.\nCheck your TOML syntax.")?;

    Ok(config)
}
