use crate::project::Project;
use std::{fs, path::Path};

impl Project
{
    pub fn path_is_valid(&self, projects_dir: &str) -> Result<(), String>
    {
        let error_message = "Cannot create project in this directory!\n";

        if !self.path.has_root()
        {
            return Err(format!("{error_message}(missing root slash)"));
        }

        if !self.path.starts_with(projects_dir)
        {
            return Err(format!("{error_message}(not in projects_dir)"));
        }

        if self.path.exists()
        {
            match fs::read_dir(&self.path)
            {
                Ok(mut entries) =>
                {
                    if entries.next().is_some()
                    {
                        return Err(format!("{error_message}(dir exists and isnt empty)"));
                    }
                }
                Err(_) =>
                {
                    return Err(format!("{error_message}(could not validate dir)"));
                }
            }
        }

        let path = &self.path;

        for p in path.ancestors()
        {
            match fs::metadata(p)
            {
                Ok(metadata) if metadata.permissions().readonly() =>
                {
                    return Err(format!("{error_message}(permission denied)"));
                }

                _ => (),
            }
        }

        Ok(())
    }

    pub fn is_project_path(path: &Path) -> bool
    {
        if path.join("projman.toml").is_file()
        {
            return true;
        }

        false
    }
}
