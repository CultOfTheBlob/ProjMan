use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
    templates::template_config::File,
};

use std::fs::write;

impl AppState
{
    pub fn update_project(&mut self) -> Result<(), Error>
    {
        let index = match self.selected_project
        {
            Some(selected) => selected,
            None => return Err(Error::Other(String::from("Error: No prject selected"))),
        };

        let project: &Project = &self.project_list[index];

        let project_files: &Vec<File> = &project.template.config().files;

        for file in project_files
        {
            if let Err(err) = write(
                project.path.join(&file.path),
                file.formatted(&project.name, &project.repo),
            )
            {
                return Err(error!(Error::Create, "project files", err));
            };
        }

        Ok(())
    }
}
