
use clap::Subcommand;
pub use clean::CleanArgs;
pub use new::NewArgs;
pub use build::{BuildArgs, Profile as BuildProfile};
pub use run::RunArgs;
pub use args::Args;

mod new;
mod project_type;
mod build;
mod run;
mod clean;
mod args;

#[derive(Subcommand, Debug)]
pub enum Commands {
    
    /// Initialize a new C++ project in the current working directory.
    Init,

    /// Create a new C++ project in a specified directory within the current working directory.
    New(NewArgs),

    /// Compile an Embargo project.
    Build(BuildArgs),

    /// Compile and run an Embargo project.
    Run(RunArgs),

    /// Clean build artifacts (default behavior is clean entire build directory)
    Clean(CleanArgs),

    //Install,
    //Uninstall,
    //Add,
}

impl Commands {
    pub fn debug_new(project_name: &str) -> Self {
        Self::New(NewArgs::with_name(project_name))
    }

    pub fn debug_build() -> Self {
        Self::Build(BuildArgs::default())
    }
}