
use crate::{commands::RunArgs, embargo_toml::GlobalEmbargoFile, error::EmbargoResult};

use super::build_project;


pub fn run_project(args: RunArgs, global_file: &GlobalEmbargoFile) -> EmbargoResult {

    build_project(*args.build_args(), global_file)?;

    //let run = Command::new();

    Ok(None)
}