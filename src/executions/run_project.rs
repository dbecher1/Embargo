
use std::{path::{Path, PathBuf}, process::Command};

use colored::Colorize;

use crate::{commands::{BuildProfile, RunArgs}, embargo_toml::{EmbargoFile, GlobalEmbargoFile}, error::EmbargoResult};

use super::build_project;

pub fn run_project(run_args: RunArgs, global_file: &GlobalEmbargoFile, embargo_toml: &EmbargoFile, embargo_toml_path: &Path) -> EmbargoResult {

    build_project(run_args.build_args, global_file, embargo_toml, embargo_toml_path)?;

    // we have where Embargo.toml is, find the path to the executable
    let mut exec_path = embargo_toml_path.to_path_buf();

    // for the displ
    let mut final_run_path = PathBuf::new();
    
    // If overrides exist in the local Embargo.toml
    if let Some(build) = &embargo_toml.package.build_path {
        exec_path.push(build);
        final_run_path.push(build);
    } else {
        exec_path.push(&global_file.build_path);
    }

    match run_args.build_args.profile {
        BuildProfile::Debug => {
            if let Some(debug_override) = &embargo_toml.package.target_path_debug {
                exec_path.push(debug_override);
            } else {
                exec_path.push(&global_file.target_path_debug);
            }
        },

        BuildProfile::Release => {
            if let Some(release_override) = &embargo_toml.package.target_path_release {
                exec_path.push(release_override);
            } else {
                exec_path.push(&global_file.target_path_release);
            }
        }
    }

    if let Some(bin_override) = &embargo_toml.package.bin_path {
        exec_path.push(bin_override);
    } else {
        exec_path.push(&global_file.bin_path);
    }

    //if let Some(p) = &embargo_toml.package
    
    // copy the cwd for the file before adding the filename
    let exec_cwd = exec_path.clone();

    // then get the name of the executable from the toml file
    exec_path.push(&embargo_toml.package.name);

    // if args are provided in Embargo.toml, add them first before the passed args
    let args = if let Some(toml_args) = &embargo_toml.package.args {
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
    println!("{} \"{}\"", "Running".green().bold(), exec_path.display());

    let mut run = Command::new(exec_path);
    let run = run
        .current_dir(exec_cwd)
        .args(args);

    run.spawn()?;

    Ok(None)
}