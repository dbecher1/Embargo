
use clap::Subcommand;
pub use new::NewArgs;

mod new;
mod project_type;

#[derive(Subcommand, Debug)]
pub enum Commands {
    
    /// Initialize a new C++ project in the current working directory.
    Init,

    /// Create a new C++ project in a specified directory within the current working directory.
    New(NewArgs),

    /// Compile an Embargo project.
    Build,
    Install,
    Uninstall,
    Add,
    Run,
}