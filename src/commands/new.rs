
use clap::Args;

use super::project_type::ProjectType;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The name of the project to create. This will also be the name of the directory created.
    name: String,

    #[command(flatten)]
    project_type: ProjectType,
}

impl NewArgs {
    pub fn name(&self) -> &str {
        &self.name
    }
}
