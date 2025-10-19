use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct CleanArgs {
    /// Clean the debug build directory
    #[arg(short, long)]
    pub debug: bool,

    /// Clean the release build directory
    #[arg(short, long)]
    pub release: bool,

    /// Clean out object files
    #[arg(short, long)]
    pub objects: bool,
}