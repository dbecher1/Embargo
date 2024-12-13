
use serde::{Serialize, Deserialize};

mod package;
mod dependencies;
mod global_file;
mod set_env;
pub mod const_values;

use package::EmbargoPackageConfig;
use dependencies::EmbargoDependenciesConfig;
pub use global_file::GlobalEmbargoFile;

/// The struct representing the Embargo.toml in every Embargo project.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EmbargoFile {
    pub package: EmbargoPackageConfig,
    pub dependencies: Option<EmbargoDependenciesConfig>,
}
