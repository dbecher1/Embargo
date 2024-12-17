use clap::Args;


#[derive(Args, Clone, Copy, Debug)]
pub struct BuildArgs {
    /// Build args are not yet implemented
    #[arg(short)]
    todo: Option<bool>,
}

impl Default for BuildArgs {
    fn default() -> Self {
        let todo = None;

        Self {
            todo,
        }
    }
}