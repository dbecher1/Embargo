
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

    pub fn with_name(project_name: &str) -> Self {
        let name = project_name.to_owned();
        let project_type = ProjectType::default();
        Self {
            name,
            project_type
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
