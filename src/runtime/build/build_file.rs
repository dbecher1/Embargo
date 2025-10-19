use std::{cell::RefCell, path::PathBuf};

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use super::{cxx_file::CxxFile, serde_helpers::*};


/// Struct representing the Embargo.build file that goes inside every build folder
/// Will likely be lifted into a parent directory at some point - not just used for build (like add, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbargoBuildFile {
    #[serde(serialize_with = "serialize_modified", deserialize_with = "deserialize_modified")]
    pub embargo_toml_modified: u64,
    pub source_files: AHashMap<PathBuf, RefCell<CxxFile>>,
}

impl EmbargoBuildFile {

    pub fn new() -> Self {
        let embargo_toml_modified = 0;
        let source_files = AHashMap::new();

        Self {
            embargo_toml_modified,
            source_files,
        }
    }
}