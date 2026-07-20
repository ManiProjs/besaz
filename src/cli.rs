use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "besaz")]
#[command(about = "A modern Rust build system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a task
    Run { name: String },

    /// List tasks
    List,

    /// Watch files and rerun task
    Watch { name: String },

    /// Create a new Besaz.toml
    Init,
}
