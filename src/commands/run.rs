
use clap::Args;

use super::BuildArgs;


#[derive(Args, Debug)]
pub struct RunArgs {

    #[command(flatten)]    
    pub(crate) build_args: BuildArgs,
    
    /// Args to pass to the  program
    #[arg(last(true))]
    pub(crate) args: Vec<String>,
}
