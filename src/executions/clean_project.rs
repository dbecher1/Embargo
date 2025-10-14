use std::{fs, path::Path};

use crate::{commands::CleanArgs, embargo_toml::{ConfigFile, EmbargoFile}, error::EmbargoResult};

// Have to polish up error handling by quite a bit

#[allow(unreachable_code)]
pub fn clean_project(args: CleanArgs, embargo_toml: &EmbargoFile, embargo_toml_path: &Path) -> EmbargoResult {

    let mut clean_path = embargo_toml_path.to_path_buf();

    // set the build path
    clean_path.push(embargo_toml.build_path());

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
            clean_path.push(embargo_toml.target_path_debug());

            // if debug and objects flags are set, clean the object artifacts in the debug target only
            if args.objects {
                clean_path.push(embargo_toml.object_path());
            } else {
                // ???
            }
        }

        if args.release {
            clean_path.push(embargo_toml.target_path_release());
        }

        message
    };

    Ok(Some(message))
}