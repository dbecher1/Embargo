use crate::cli::Args;
use embargo_toml::EmbargoFile;
use error::EmbargoResult;
use log::debug;
use std::path::Path;

pub mod cli;
mod embargo_toml;
mod error;
mod runtime;

pub fn run(args: Args, temp_dir: Option<&Path>) -> EmbargoResult {

    let read_file = args.read_file();

    let (embargo_toml, embargo_toml_path) = if read_file {
        let (temp1, temp2) = EmbargoFile::read_file(temp_dir)?;
        debug!("Embargo.toml read: {:?}\nPath: {:?}", temp1, temp2);
        (Some(temp1), Some(temp2))
    } else {
        (None, None)
    };

    runtime::run(&args, embargo_toml, embargo_toml_path, read_file, temp_dir)
}

