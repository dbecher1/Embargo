use clap::Args;

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct ProjectType {
    
    /// Create a new executable project. This is the default behavior. 
    #[arg(long)]
    bin: bool,

    /// Create a new library project. TODO
    #[arg(long)]
    lib: bool,
}

impl Default for ProjectType {
    fn default() -> Self {
        Self {
            bin: true,
            lib: false
        }
    }
}