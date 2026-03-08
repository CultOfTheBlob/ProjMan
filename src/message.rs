use std::path::PathBuf;

use crate::state::{project::Project, project_type::ProjectType};

#[derive(Debug, Clone)]
pub enum Message
{
    Open(Project),
    Select(usize),
    Deselect,
    Remove,
    ConfirmRemove,
    CancelRemove,
    ToggleRemoveProjectFolder(bool),
    Create,
    ConfirmCreate,
    CreateProjectDir(Result<String, String>),
    CloneProjectRepo(Result<String, String>),
    CreateProjmanFile(Result<String, String>),
    CreateDirStructure(Result<String, String>),
    CreateProjectFiles(Result<String, String>),
    ExecuteBuildCommand(usize, Result<String, String>),
    FinishCreate(Result<Vec<Project>, String>),
    CancelCreate,
    ChangeNewProjectName(String),
    ChangeNewProjectType(ProjectType),
    ChangeNewProjectRepo(String),
    ChangeNewProjectPath(PathBuf),
    Import,
    RestoreNonexistant,
    RemoveNonexistant,
    FinishRemoveNonexistant(Result<usize, String>),
}
