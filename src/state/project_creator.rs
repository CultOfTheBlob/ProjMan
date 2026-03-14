use std::{
    fs::{self, create_dir, create_dir_all, read_to_string, remove_dir, write},
    io::Write,
    path::PathBuf,
    process::{self},
};

use directories::ProjectDirs;

use crate::{
    state::{project::Project, project_type::ProjectType},
    templates::{Command, File, Folder},
};

pub async fn create_project_dir(project_path: PathBuf) -> Result<String, String>
{
    match create_dir_all(&project_path)
    {
        Ok(_) => Ok("Created project dir...".to_string()),
        Err(err) => Err(format!("Could not create project directory ({err})")),
    }
}

pub async fn clone_project_repo(project: Project) -> Result<String, String>
{
    match project.clone_repo()
    {
        Ok(_) => Ok("Cloned project repo...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(format!("Could not clone project repo ({err})"))
        }
    }
}

pub async fn create_projman_file(
    project_path: PathBuf,
    project_type: ProjectType,
) -> Result<String, String>
{
    match fs::File::create_new(project_path.join(".projman"))
    {
        Ok(mut file) =>
        {
            if let Err(err) = file.write_all(project_type.to_string().as_bytes())
            {
                return Err(format!("Could not create .projman file {err}"));
            }
        }
        Err(err) => return Err(format!("Could not create .projman file {err}")),
    };

    Ok("Created .projman file...".to_string())
}

pub async fn create_dir_structure(
    project_dir_structure: Vec<Folder>,
    project_path: PathBuf,
) -> Result<String, String>
{
    for dir in &project_dir_structure
    {
        let dirs: Vec<PathBuf> = dir.parse(&project_path);

        for dir in &dirs
        {
            if let Err(err) = create_dir(dir)
            {
                return Err(format!("Could not create directory structure ({err})"));
            }
        }
    }

    Ok("Created project directory structure...".to_string())
}

pub async fn create_project_files(
    project_files: Vec<File>,
    project_path: PathBuf,
) -> Result<String, String>
{
    for file in &project_files
    {
        if let Err(err) = write(project_path.join(&file.path), &file.content)
        {
            return Err(format!("Could not create project files ({err})"));
        };
    }

    Ok("Created project files...".to_string())
}

pub async fn execute_build_command(
    command: Command,
    project_path: PathBuf,
) -> Result<String, String>
{
    match process::Command::new(&command.program)
        .args(&command.args)
        .current_dir(&project_path)
        .status()
    {
        Ok(_) => Ok(format!(
            "Executed [{} {:?}]...",
            command.program,
            command.args.join(" ")
        )),
        Err(err) => Err(format!(
            "Could not execute [{} {:?}] ({err})",
            command.program,
            command.args.join(" ")
        )),
    }
}

pub async fn commit_projman_init(project: Project) -> Result<String, String>
{
    match project.init_commit()
    {
        Ok(_) => Ok("Committed ProjMan init...".to_string()),

        Err(err) =>
        {
            let _ = remove_dir(&project.path);
            Err(format!("Could commit initialization ({err})"))
        }
    }
}

pub async fn add_project_to_json(project: Project) -> Result<Vec<Project>, String>
{
    if let Some(proj_dirs) = ProjectDirs::from("", "", "projman")
    {
        if let Err(err) = create_dir_all(proj_dirs.config_dir())
        {
            return Err(format!("Error: Could not create config dir ({err})"));
        };

        let config_path: PathBuf = proj_dirs.config_dir().join("projects.json");

        if !config_path.is_file()
        {
            return Err("Error: projects.json does not exist".to_string());
        }

        let projects_from_json: String = match read_to_string(&config_path)
        {
            Ok(string) => string,
            Err(err) => return Err(format!("Error: Could not read projects.json ({err})")),
        };

        let mut projects: Vec<Project> = match serde_json::from_str(&projects_from_json)
        {
            Ok(it) => it,
            Err(err) => return Err(format!("Error: Could not parse projects.json ({err})")),
        };

        projects.push(project);

        let projects_to_json = match serde_json::to_string_pretty(&projects)
        {
            Ok(it) => it,
            Err(err) => return Err(format!("Error: Could not parse projects.json ({err})")),
        };

        if let Err(err) = write(&config_path, projects_to_json.as_bytes())
        {
            return Err(format!("Error: Could not write to projects.json ({err})"));
        };

        return Ok(projects);
    }

    Err("Error: Could not find config dir".to_string())
}
