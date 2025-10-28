use crate::embargo_toml::{cfg_trait::ConfigFile, global_config::EmbargoGlobalConfig};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

// The global config object, will be filled with values from either the global file or default values
static GLOBAL_CONF: LazyLock<EmbargoGlobalConfig> =
    LazyLock::new(EmbargoGlobalConfig::try_read);

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct EmbargoPackageConfig {
    pub name: String,
    pub version: String,
    pub entry: String,
    pub author: Option<String>,
    source_path: Option<String>,       // relative to Embargo.toml
    build_path: Option<String>,        // relative to Embargo.toml
    auto_clean: Option<bool>,          // Auto-delete .o files
    object_path: Option<String>,       // directory for .o files
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
    #[allow(unused)]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

impl<'a> ConfigFile<'a> for EmbargoPackageConfig {
    fn compiler(&'a self) -> &'a str {
        GLOBAL_CONF.compiler()
    }

    fn linker(&'a self) -> &'a str {
        GLOBAL_CONF.linker()
    }

    /// Returns the package's source path if set, otherwise the global source path.
    fn source_path(&'a self) -> &'a str {
        //&self.source_path.unwrap_or_else(|| &global_conf().source_path)
        if let Some(ref s) = self.source_path {
            s
        } else {
            &GLOBAL_CONF.source_path
        }
    }

    /// Returns the package's build path if set, otherwise the global build path.
    fn build_path(&'a self) -> &'a str {
        if let Some(ref bp) = self.build_path {
            bp
        } else {
            &GLOBAL_CONF.build_path
        }
    }

    /// Returns whether auto_clean is enabled for this package. Default: false.
    /// Note: there is no `auto_clean` in the global config, so we default to false when unset.
    fn auto_clean(&self) -> bool {
        self.auto_clean.unwrap_or(false)
    }

    /// Returns the package's object path if set, otherwise the global object path.
    fn object_path(&'a self) -> &'a str {
        if let Some(ref op) = self.object_path {
            op
        } else {
            &GLOBAL_CONF.object_path
        }
    }

    /// Returns the package's debug target path if set, otherwise the global debug target path.
    fn target_path_debug(&'a self) -> &'a str {
        if let Some(ref tpd) = self.target_path_debug {
            tpd
        } else {
            &GLOBAL_CONF.target_path_debug
        }
    }

    /// Returns the package's release target path if set, otherwise the global release target path.
    fn target_path_release(&'a self) -> &'a str {
        if let Some(ref tpr) = self.target_path_release {
            tpr
        } else {
            &GLOBAL_CONF.target_path_release
        }
    }

    /// Returns the package's bin path if set, otherwise the global bin path.
    fn bin_path(&'a self) -> &'a str {
        if let Some(ref bp) = self.bin_path {
            bp
        } else {
            &GLOBAL_CONF.bin_path
        }
    }

    /// Returns the package's lib path if set, otherwise the global lib path.
    fn lib_path(&'a self) -> &'a str {
        if let Some(ref lp) = self.lib_path {
            lp
        } else {
            &GLOBAL_CONF.lib_path
        }
    }

    /// Returns compiler flags for this package. If unset, returns an empty Vec.
    fn flags(&'a self) -> &'a [String] {
        if let Some(ref f) = self.flags {
            f
        } else {
            &[]
        }
    }

    /// Returns runtime args for this package. If unset, returns an empty Vec.
    fn args(&'a self) -> &'a [String] {
        if let Some(ref a) = self.args {
            a
        } else {
            &[]
        }
    }

    /// Returns the package author, if present.
    fn author(&self) -> Option<&str> {
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

