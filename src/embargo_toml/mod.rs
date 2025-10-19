
use std::{env, fs, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

mod pkg_config;
mod dependencies;
mod global_config;
mod toolchain;
mod cfg_trait;

#[allow(unused)]
pub mod const_values;

pub use cfg_trait::ConfigFile;
use pkg_config::EmbargoPackageConfig;
use dependencies::EmbargoDependenciesConfig;
use crate::error::EmbargoError;

/// The struct representing the Embargo.toml in every Embargo project.
#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct EmbargoFile {
    pub package: EmbargoPackageConfig,
    pub dependencies: Option<EmbargoDependenciesConfig>,
}

impl EmbargoFile {

    /// If the file is found, returns a tuple of the file struct and the path where the file was located
    /// Param dir will be Some if this is a test; in that instance, it will be the path of the Temp dir
    /// Otherwise the cwd will be used
    pub fn read_file(dir: Option<&Path>) -> Result<(Self, PathBuf), EmbargoError> {

        // Use the cwd in normal runs
        // If this is a test, create a temp dir and
        let cwd = if let Some(d) = dir {
            let file = Self::default();
            let mut path = d.to_owned();
            path.push("Embargo.toml");
            return Ok((file, path));
        } else {
            env::current_dir()?
        };

        let mut path = match find_embargo_file_path(&cwd) {
            Some(p) => p,
            None => {
                return Err(EmbargoError::new("Unable to locate Embargo.toml"));
            }
        };
        let file = fs::read_to_string(&path)?;
        path.pop(); // remove the file name
        Ok((toml::from_str(&file)?, path))
    }
   
}

impl<'a> ConfigFile<'a> for EmbargoFile {
    fn compiler(&'a self) -> &'a str {
        self.package.compiler()
    }

    fn linker(&'a self) -> &'a str {
        self.package.linker()
    }

    fn source_path(&'a self) -> &'a str {
        self.package.source_path()
    }

    fn build_path(&'a self) -> String {
        self.package.build_path()
    }

    fn auto_clean(&'a self) -> bool {
        self.package.auto_clean()
    }

    fn object_path(&'a self) -> String {
        self.package.object_path()
    }

    fn target_path_debug(&'a self) -> String {
        self.package.target_path_debug()
    }

    fn target_path_release(&'a self) -> String {
        self.package.target_path_release()
    }

    fn bin_path(&'a self) -> String {
        self.package.bin_path()
    }

    fn lib_path(&'a self) -> String {
        self.package.lib_path()
    }

    fn flags(&'a self) -> Vec<String> {
        self.package.flags()
    }

    fn args(&'a self) -> Vec<String> {
        self.package.args()
    }

    fn author(&'a self) -> Option<&'a str> {
        self.package.author()
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