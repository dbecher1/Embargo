use std::{cell::RefCell, path::PathBuf};

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use super::cxx_file::CxxFile;


/// Struct representing the Embargo.build file that goes inside every build folder
/// Will likely be lifted into a parent directory at some point - not just used for build (like add, etc)
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbargoBuildFile {
    pub source_files: AHashMap<PathBuf, RefCell<CxxFile>>,
}

impl EmbargoBuildFile {

    pub fn new() -> Self {
        let source_files = AHashMap::new();

        Self {
            source_files,
        }
    }
}