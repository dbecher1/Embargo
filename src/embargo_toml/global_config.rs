use std::{env, fs, path::PathBuf};

use log::{debug, error};
use serde::Deserialize;
use super::toolchain::Toolchain;

static ENV_VAR_NAME: &'static str = "EMBARGO_HOME";
static GLOBAL_FILE_NAME: &'static str = "EmbargoGlobal.toml";

fn git_default() -> bool {
    true
}

fn cxx_default() -> String {
    "c++17".to_owned()
}

fn source_path_default() -> String {
    "src".to_owned()
}

fn build_path_default() -> String {
    "target".to_owned()
}

fn target_debug_default() -> String {
    "debug".to_owned()
}

fn target_release_default() -> String {
    "release".to_owned()
}

fn object_default() -> String {
    ".o".to_owned()
}

fn bin_default() -> String {
    "bin".to_owned()
}

fn lib_default() -> String {
    "lib".to_owned()
}

#[derive(Deserialize, Debug, Hash)]
pub struct EmbargoGlobalConfig {
    #[serde(default = "git_default")]
    use_git: bool,

    #[serde(default)]
    toolchain: Toolchain,

    #[serde(default = "cxx_default")]
    cxx_version: String,

    #[serde(default = "source_path_default")]
    pub source_path: String,

    #[serde(default = "build_path_default")]
    pub build_path: String,

    #[serde(default = "target_debug_default")]
    pub target_path_debug: String,

    #[serde(default = "target_release_default")]
    pub target_path_release: String,

    #[serde(default = "object_default")]
    pub object_path: String,

    #[serde(default = "bin_default")]
    pub bin_path: String,

    #[serde(default = "lib_default")]
    pub lib_path: String,
}

impl Default for EmbargoGlobalConfig {
    fn default() -> Self {
        Self {
            use_git: git_default(),
            toolchain: Default::default(),
            cxx_version: cxx_default(),
            source_path: source_path_default(),
            build_path: build_path_default(),
            target_path_debug: target_debug_default(),
            target_path_release: target_release_default(),
            object_path: object_default(),
            bin_path: bin_default(),
            lib_path: lib_default(),
        }
    }
}

impl EmbargoGlobalConfig {

    pub fn compiler(&self) -> &str {
        &self.toolchain.compiler()
    }

    pub fn linker(&self) -> &str {
        self.toolchain.linker()
    }

    /// Attempts to read the global embargo file
    /// This will be either:
    /// located in a default expected path by OS (TODO), OR
    /// located within the directory specified by the environment variable EMBARGO_HOME
    /// If the file is not found, Embargo will use default values
    pub fn try_read() -> Self{

        // If in debug mode, try toread the global file from the local one
        let embargo_home = match env::var(ENV_VAR_NAME) {
            Ok(env) => env,
            Err(_) => {
                debug!("EMBARGO_HOME environment variable is not set; using default parameters");
                return Self::default()
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
                        Err(e) => {
                            error!("{}", e);
                            error!("Error reading Embargo global file - bad field indicated\nUsing default config values");
                            return Self::default()
                        }
                    }
                },
                Err(_) => {
                    return Self::default()
                },
            }
        };
        debug!("Successfully read {}", GLOBAL_FILE_NAME);
        file
    }
}
