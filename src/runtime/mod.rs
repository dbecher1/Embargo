
mod create_project;
mod build;
mod run_project;
mod clean_project;

use std::path::{Path, PathBuf};

pub use create_project::*;
pub use build::build_project;
use log::debug;
pub use run_project::run_project;
pub use clean_project::clean_project;

use crate::{cli::Args, embargo_toml::EmbargoFile, error::EmbargoResult};

pub fn run(args: &Args, embargo_toml: Option<EmbargoFile>, embargo_toml_path: Option<PathBuf>, read_file: bool, temp_dir: Option<&Path>) -> EmbargoResult {
    use crate::cli::Commands::*;
    match args.command() {
        Init => {
            debug!("Command executed: Init");
            // Todo - probably want to make some init args, like project type etc
            create_project(None, None)
        },

        New(new_args) => {
            debug!("Command executed: New\nArgs: {:?}", new_args);
            create_project(Some(new_args), temp_dir)
        },

        // With the match guard, unwrap below is safe

        Build(build_args) if read_file => {
            debug!("Command executed: Build");
            build_project(build_args, &embargo_toml.unwrap(), &embargo_toml_path.unwrap())
        },
        
        Run(run_args) if read_file => {
            debug!("Command executed: Run");
            run_project(run_args, &embargo_toml.unwrap(), &embargo_toml_path.unwrap())
        },

        Clean(clean_args) if read_file => {
            debug!("Command executed: Clean");
            clean_project(clean_args, &embargo_toml.unwrap(), &embargo_toml_path.unwrap())
        },

        _ => unreachable!(),
    }
}