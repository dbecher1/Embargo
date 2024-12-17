use std::{env, fs, path::PathBuf};

use log::debug;
use serde::Deserialize;

use crate::{embargo_toml::const_values::GLOBAL_FILE_NAME, error::EmbargoError};

use super::const_values::ENV_VAR_NAME;


#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct GlobalEmbargoFile {
    use_git: bool,
    cxx_compiler: String,
    cxx_version: String,
    c_compiler: String,
    pub source_path: String,
    pub build_path: String,
    pub bin_path: String,
}

impl GlobalEmbargoFile {

    pub fn cxx_compiler(&self) -> &str {
        &self.cxx_compiler
    }

    /// Attempts to read the global embargo file, located within the directory specified by the environment variable EMBARGO_HOME
    pub fn try_read() -> Result<Self, EmbargoError> {

        // If in debug mode, read the global file from the local one
        let embargo_home = if cfg!(debug_assertions) {
            "template_files".to_owned()
        } else {
            match env::var(ENV_VAR_NAME) {
                Ok(env) => env,
                Err(_) => {
                    return Err(EmbargoError::new("EMBARGO_HOME environment variable is not set."));
                }
            }
        };

        debug!("Reading EmbargoGlobal.toml from environment variable: {}", embargo_home);

        let file: Self = {
            let mut global_file_path = PathBuf::from(embargo_home);
            global_file_path.push(GLOBAL_FILE_NAME);

            // debug!("Global file path: {}", global_file_path.display());

            match fs::read(&global_file_path) {
                Ok(f) => {

                    debug!("Found global file at {}", global_file_path.display());

                    let file_contents = String::from_utf8_lossy(&f);

                    match toml::from_str(&file_contents) {
                        Ok(toml) => toml,
                        Err(_) => {
                            return Err(EmbargoError::new("could not deserialize global config from TOML"));
                        }
                    }
                },
                Err(_) => {
                    return Err(EmbargoError::new("unable to open global file."));
                },
            }
        };
        debug!("Successfully read {}", GLOBAL_FILE_NAME);
        Ok(file)
    }
}
