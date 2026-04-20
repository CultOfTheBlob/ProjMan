use crate::{
    error::{Error, ErrorInfo},
    state::app_state::AppState,
};
use std::fs;

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

            let file = file.formatted(&project.name, &project.repo, &project.license);

            if let Err(err) = fs::write(project.path.join(&file.path), &file.content)
            {
                return Err(error!(Error::Create, "project files", err));
            }
        }

        Ok(())
    }
}
