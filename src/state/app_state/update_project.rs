use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
};
use std::fs::write;

impl AppState
{
    pub fn update_project(&self) -> Result<(), Error>
    {
        let Some(index) = self.selected_project
        else
        {
            return Err(Error::Other(String::from("Error: No prject selected")));
        };

        let project = &self.project_list[index];

        let project_files = &project.template.config().files;

        for file in project_files
        {
            if !file.tracked
            {
                continue;
            }

            if let Err(err) = write(
                project.path.join(&file.path),
                file.formatted(&project.name, &project.repo, &project.license),
            )
            {
                return Err(error!(Error::Create, "project files", err));
            }
        }

        Ok(())
    }
}
