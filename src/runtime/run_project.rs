
use std::{path::{Path, PathBuf}, process::Command};

use colored::Colorize;

use crate::cli::{BuildProfile, RunArgs};
use crate::embargo_toml::{ConfigFile, EmbargoFile};
use crate::error::EmbargoResult;
use super::build_project;

pub fn run_project(run_args: &RunArgs, embargo_toml: &EmbargoFile, embargo_toml_path: &Path) -> EmbargoResult {

    build_project(&run_args.build_args, embargo_toml, embargo_toml_path)?;

    // we have where Embargo.toml is, find the path to the executable
    let mut exec_path = embargo_toml_path.to_path_buf();

    // for the displ
    let mut final_run_path = PathBuf::new();
    
    exec_path.push(embargo_toml.build_path());
    final_run_path.push(embargo_toml.build_path());

    match run_args.build_args.profile {
        BuildProfile::Debug => {
            exec_path.push(embargo_toml.target_path_debug());
        },

        BuildProfile::Release => {
            exec_path.push(embargo_toml.target_path_release());
        }
    }
    exec_path.push(embargo_toml.bin_path());

    //if let Some(p) = &embargo_toml.package
    
    // copy the cwd for the file before adding the filename
    let exec_cwd = exec_path.clone();

    // then get the name of the executable from the toml file
    exec_path.push(&embargo_toml.package.name);

    // if args are provided in Embargo.toml, add them first before the passed args
    // FIXME
    let args: Vec<&str> = vec![];
    /*
    let args = if let Some(toml_args) = &embargo_toml.args() {
        // if args were passed, do the above
        if !run_args.args.is_empty() {
            // have to clone these to make them owned, not super ideal but eh
            let mut toml_args = toml_args.clone();
            let mut passed_args = run_args.args.clone();
            toml_args.append(&mut passed_args);
            toml_args
        } else {
            // no args passed on calling, just use the args from the toml file
            toml_args.clone()
        }
    } else {
        // no args in Embargo.toml, check to see if any were passed
        if !run_args.args.is_empty() {
            run_args.args
        } else {
            vec![]
        }
    };
     */
    println!("{} \"{}\"", "Running".green().bold(), exec_path.display());

    let mut run = Command::new(exec_path);
    let run = run
        .current_dir(exec_cwd)
        .args(args);

    run.spawn()?;

    Ok(None)
}