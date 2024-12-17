
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbargoPackageConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub author: Option<String>,
    pub source_path: Option<String>, // relative to Embargo.toml
    pub build_path: Option<String>, // relative to Embargo.toml
    pub auto_clean: Option<bool>, // Auto-delete .o files
    pub object_path: Option<String>, // directory for .o files
    pub bin_path: Option<String>, // Relative to build path
    pub flags: Option<Vec<String>>, // Compiler flags
}

impl EmbargoPackageConfig {

    #[allow(unused)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

impl Default for EmbargoPackageConfig {
    fn default() -> Self {
        Self {
            name: "test".to_owned(),
            version: "0.1.0".to_owned(),
            entry: "main.cpp".to_owned(),
            author: None,
            source_path: None,
            build_path: None,
            auto_clean: None,
            object_path: None,
            bin_path: None,
            flags: None,
        }
    }
}