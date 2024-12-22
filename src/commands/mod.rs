
use clap::Subcommand;
pub use clean::CleanArgs;
pub use new::NewArgs;
pub use build::{BuildArgs, Profile as BuildProfile};
pub use run::RunArgs;

mod new;
mod project_type;
mod build;
mod run;
mod clean;

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