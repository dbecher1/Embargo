use std::path::Path;
use embargo_toml::EmbargoFile;
use error::EmbargoResult;
use cli::Commands;
use log::debug;
use crate::cli::Args;

pub mod cli;
mod embargo_toml;
mod runtime;
mod error;

pub fn run(args: Args, temp_dir: Option<&Path>) -> EmbargoResult {
    use Commands::*;

    let read_file = !matches!(args.command(), Init | New(_));

    let (embargo_toml, embargo_toml_path) = if read_file {
        let (temp1, temp2) = EmbargoFile::read_file(temp_dir)?;
        debug!("Embargo.toml read: {:?}\nPath: {:?}", temp1, temp2);
        (Some(temp1), Some(temp2))
    } else {
        (None, None)
    };

    runtime::run(&args, embargo_toml, embargo_toml_path, read_file, temp_dir)
}