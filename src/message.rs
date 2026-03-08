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
    ProgressCreate(String),
    FinishCreate(Result<Vec<Project>, String>),
    CancelCreate,
    FinishBuildCommand(usize, bool),
    ChangeNewProjectName(String),
    ChangeNewProjectType(ProjectType),
    ChangeNewProjectRepo(String),
    ChangeNewProjectPath(PathBuf),
    Import,
    RestoreNonexistant,
    RemoveNonexistant,
    FinishRemoveNonexistant(Result<usize, String>),
}
