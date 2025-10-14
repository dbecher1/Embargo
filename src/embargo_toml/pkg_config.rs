use std::sync::OnceLock;
use serde::{Serialize, Deserialize};
use crate::embargo_toml::global_config::EmbargoGlobalConfig;

// The global config object, will be filled with values from either the global file or default values
static GLOBAL_CONF: OnceLock<EmbargoGlobalConfig> = OnceLock::new();

fn global_conf() -> &'static EmbargoGlobalConfig {
    GLOBAL_CONF.get_or_init(|| EmbargoGlobalConfig::try_read())
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct EmbargoPackageConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub author: Option<String>,
    source_path: Option<String>, // relative to Embargo.toml
    build_path: Option<String>, // relative to Embargo.toml
    auto_clean: Option<bool>, // Auto-delete .o files
    object_path: Option<String>, // directory for .o files
    target_path_debug: Option<String>, // Relative to build path
    target_path_release: Option<String>,
    bin_path: Option<String>,
    lib_path: Option<String>,

    flags: Option<Vec<String>>, // Compiler flags

    // args to pass to the process
    // they are passed first, before any given args
    args: Option<Vec<String>>,
}

impl EmbargoPackageConfig {

     pub fn compiler(&self) -> &str {
        global_conf().toolchain.compiler()
    }

    pub fn linker(&self) -> &str {
        global_conf().toolchain.linker()
    }

    #[allow(unused)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// Returns the package's source path if set, otherwise the global source path.
    pub fn source_path(&self) -> String {
        self.source_path.clone().unwrap_or_else(|| global_conf().source_path.clone())
    }

    /// Returns the package's build path if set, otherwise the global build path.
    pub fn build_path(&self) -> String {
        self.build_path.clone().unwrap_or_else(|| global_conf().build_path.clone())
    }

    /// Returns whether auto_clean is enabled for this package. Default: false.
    /// Note: there is no `auto_clean` in the global config, so we default to false when unset.
    pub fn auto_clean(&self) -> bool {
        self.auto_clean.unwrap_or(false)
    }

    /// Returns the package's object path if set, otherwise the global object path.
    pub fn object_path(&self) -> String {
        self.object_path.clone().unwrap_or_else(|| global_conf().object_path.clone())
    }

    /// Returns the package's debug target path if set, otherwise the global debug target path.
    pub fn target_path_debug(&self) -> String {
        self.target_path_debug.clone().unwrap_or_else(|| global_conf().target_path_debug.clone())
    }

    /// Returns the package's release target path if set, otherwise the global release target path.
    pub fn target_path_release(&self) -> String {
        self.target_path_release.clone().unwrap_or_else(|| global_conf().target_path_release.clone())
    }

    /// Returns the package's bin path if set, otherwise the global bin path.
    pub fn bin_path(&self) -> String {
        self.bin_path.clone().unwrap_or_else(|| global_conf().bin_path.clone())
    }

    /// Returns the package's lib path if set, otherwise the global lib path.
    pub fn lib_path(&self) -> String {
        self.lib_path.clone().unwrap_or_else(|| global_conf().lib_path.clone())
    }

    /// Returns compiler flags for this package. If unset, returns an empty Vec.
    pub fn flags(&self) -> Vec<String> {
        self.flags.clone().unwrap_or_default()
    }

    /// Returns runtime args for this package. If unset, returns an empty Vec.
    pub fn args(&self) -> Vec<String> {
        self.args.clone().unwrap_or_default()
    }

    /// Returns the package author, if present.
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
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
            target_path_debug: None,
            target_path_release: None,
            bin_path: None,
            lib_path: None,
            flags: None,
            args: None,
        }
    }
}