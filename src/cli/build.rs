use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Profile {
    Debug,
    Release
}

#[derive(Args, Clone, Copy, Debug)]
pub struct BuildArgs {
    /// Optimization level of the build (not yet implemented)
    #[arg(value_enum, default_value_t = Profile::Debug)]
    pub profile: Profile,
}

impl Default for BuildArgs {
    fn default() -> Self {
        let profile = Profile::Debug;

        Self {
            profile
        }
    }
}