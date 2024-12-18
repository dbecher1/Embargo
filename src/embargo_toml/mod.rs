
use std::{env, fs, path::{Path, PathBuf}};

use serde::{Serialize, Deserialize};

mod package;
mod dependencies;
mod global_file;
mod toolchain;

#[allow(unused)]
pub mod const_values;

use package::EmbargoPackageConfig;
use dependencies::EmbargoDependenciesConfig;
pub use global_file::GlobalEmbargoFile;

use crate::error::EmbargoError;

/// The struct representing the Embargo.toml in every Embargo project.
#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct EmbargoFile {
    pub package: EmbargoPackageConfig,
    pub dependencies: Option<EmbargoDependenciesConfig>,
}

impl EmbargoFile {

    /// If the file is found, returns a tuple of the file struct and the path where the file was located
    pub fn read_file() -> Result<(Self, PathBuf), EmbargoError> {

        let mut cwd = env::current_dir()?;

        if cfg!(debug_assertions) {
            cwd.push(".test_build/src/test");
        }

        let path = match find_embargo_file_path(&cwd) {
            Some(p) => p,
            None => {
                return Err(EmbargoError::new("Unable to locate Embargo.toml"));
            }
        };

        let file = fs::read_to_string(&path)?;
        Ok((toml::from_str(&file)?, path))
    }
   
}

/// Tries to find the Embargo.toml file
/// If not in cwd, recursively searches parent directories until found
fn find_embargo_file_path(path: &Path) -> Option<PathBuf> {

    if let Ok(entry) = path.read_dir() {

        return match entry
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().eq_ignore_ascii_case("Embargo.toml")) {
                Some(entry) => {
                    Some(entry.path())
                },
                None => {
                    let mut p = path.to_path_buf();
                    if p.pop() {
                        // recurse
                        find_embargo_file_path(&p)
                    } else {
                        None
                    }
                }
        }
    }
    // I don't think this should be reachable but leaving it as a safeguard
    None
} 