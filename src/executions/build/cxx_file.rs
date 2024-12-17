use std::{path::{Path, PathBuf}, process, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::error::EmbargoError;
use super::serde_helpers::*;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CxxFileType {
    Source,
    Header,
}

fn modified_default() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CxxFile {
    pub file_type: CxxFileType,

    // flag indicating file has been changed
    #[serde(skip, default = "modified_default")]
    changed: bool, 

    #[serde(serialize_with = "serialize_modified", deserialize_with = "deserialize_modified")]
    modified: u64,

    dependencies: Vec<PathBuf>,

    pub dependees: Vec<PathBuf>,
}

impl CxxFile {

    pub fn new(file_type: CxxFileType, modified: u64, path: &Path) -> Result<Self, EmbargoError> {
        let changed = false;
        let dependencies = match gen_deps(path) {
            Ok(dep) => dep,
            Err(e) => {
                return Err(EmbargoError::new(&e))
            }
        };
        let dependees = Vec::new();

        Ok(Self {
            file_type,
            changed,
            modified,
            dependencies,
            dependees
        })
    }

    pub fn modified(&self) -> u64 {
        self.modified
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn set_changed(&mut self) {
        self.changed = true;
    }

    pub fn dependencies(&self) -> &[PathBuf] {
        self.dependencies.as_slice()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectFile {
    #[serde(serialize_with = "serialize_modified", deserialize_with = "deserialize_modified")]
    modified: u64,
}

fn gen_deps(path: &Path) -> Result<Vec<PathBuf>, String> {

    // TODO: get compiler

    let output = process::Command::new("g++")
        .arg(path.to_str().unwrap_or_default())
        .args(["-MM", "-MG"]).output();

    return match output {
        Ok(output) => {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                let o = s.split_whitespace()
                    .enumerate()
                    .filter(|(i, s)| (*i > 2) && (*s != "\\")) // skip first 2 entries
                    .map(|(_, p)| PathBuf::from_str(p))
                    .filter_map(|p| p.ok())
                    .collect();

                    Ok(o)
            } else {
                let s = String::from_utf8_lossy(&output.stderr);
                Err(s.to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}