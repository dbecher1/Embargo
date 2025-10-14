use std::{fs, path::Path};

use crate::{commands::CleanArgs, embargo_toml::EmbargoFile, error::EmbargoResult};

// Have to polish up error handling by quite a bit

#[allow(unreachable_code)]
pub fn clean_project(args: CleanArgs, embargo_toml: &EmbargoFile, embargo_toml_path: &Path) -> EmbargoResult {

    let mut clean_path = embargo_toml_path.to_path_buf();

    // set the build path
    if let Some(build_path) = &embargo_toml.package.build_path {
        clean_path.push(build_path);
    } else {
        clean_path.push(&global_file.build_path);
    }

    // set the output message based on what happens here
    let message = if !args.objects && !args.debug && !args.release {
        // clean the entire directory
        fs::remove_dir_all(&clean_path)?;
        String::from("Successfully cleaned build directory.")
    } else {
        unimplemented!();
        
        let mut message = String::new();

        // in lieu of a fallthrough, handle each case if the flag is set
        
        if args.debug {
            if let Some(debug_override) = &embargo_toml.package.target_path_debug {
                clean_path.push(debug_override);
            } else {
                clean_path.push(&global_file.target_path_debug);
            }

            // if debug and objects flags are set, clean the object artifacts in the debug target only
            if args.objects {
                if let Some(obj_override) = &embargo_toml.package.object_path {
                    clean_path.push(obj_override);
                } else {
                    clean_path.push(&global_file.object_path);
                }
            } else {

            }
        }

        if args.release {
            if let Some(release_override) = &embargo_toml.package.target_path_release {
                clean_path.push(release_override);
            } else {
                clean_path.push(&global_file.target_path_release);
            }
        }

        message
    };

    Ok(Some(message))
}