use clap::Args;

use super::BuildArgs;


#[derive(Args, Debug)]
pub struct RunArgs {

    #[command(flatten)]
    build_args: BuildArgs,

    /// Args to pass to the  program
    #[arg(long)]
    args: Option<Vec<String>>,
}

#[allow(unused)]
impl RunArgs {

    pub fn build_args(&self) -> &BuildArgs {
        &self.build_args
    }

    pub fn args(&self) -> Option<&Vec<String>> {
        return match &self.args {
            Some(args) => Some(args),
            None => None,
        }
    }
}