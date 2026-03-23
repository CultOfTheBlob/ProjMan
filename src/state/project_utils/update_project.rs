use crate::{
    error::{Error, ErrorInfo},
    project::Project,
    state::app_state::AppState,
    templates::File,
};

use std::fs::write;

pub fn update_project(state: &mut AppState) -> Result<(), Error>
{
    if let Some(index) = state.selected_project
    {
        let project: &Project = &state.project_list[index];

        let project_files: Vec<File> = project.project_type.template_config()?.files;

        for file in &project_files
        {
            if let Err(err) = write(project.path.join(&file.path), &file.content)
            {
                return Err(Error::Create(ErrorInfo {
                    string: String::from("project files"),
                    err: err.to_string(),
                }));
            };
        }

        return Ok(());
    }

    Ok(())
}
