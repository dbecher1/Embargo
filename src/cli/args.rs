#![allow(unused)]

use clap::Parser;
use crate::cli::Commands;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Manually set the logging level
    #[arg(long)]
    debug_log: bool,
}

impl Args {
    
    pub fn debug_new(project_name: &str) -> Self {
        let command = Commands::debug_new(project_name);
        let debug_log = false;
        Self {
            command,
            debug_log,
        }
    }

    pub fn debug_build() -> Self {
        let command = Commands::debug_build();
        let debug_log = false;
        Self {
            command,
            debug_log,
        }
    }

    /// Returns a reference to the parsed subcommand.
    pub fn command(&self) -> &Commands {
        &self.command
    }

    /// Returns whether debug logging was requested.
    pub fn debug_log(&self) -> bool {
        self.debug_log
    }
}