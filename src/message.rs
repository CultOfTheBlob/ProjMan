use std::path::PathBuf;

use crate::state::{project::Project, project_type::ProjectType};

#[derive(Debug, Clone)]
pub enum Message
{
    Opened(Project),
    Selected(usize),
    Deselected,
    Removed,
    RemoveConfirmed,
    RemoveCanceled,
    RemoveProjectFolderToggled(bool),
    Created,
    CreateConfirmed,
    ProjectDirCreated(Result<String, String>),
    ProjectRepoCloned(Result<String, String>),
    ProjmanFileCreated(Result<String, String>),
    DirStructureCreated(Result<String, String>),
    ProjectFilesCreated(Result<String, String>),
    BuildCommandExecuted(usize, Result<String, String>),
    CommitedProjmanInit(Result<String, String>),
    CreateFinished(Result<Vec<Project>, String>),
    CreateCanceled,
    NewProjectNameChanged(String),
    NewProjectTypeChanged(ProjectType),
    NewProjectRepoChanged(String),
    NewProjectPathChanged(PathBuf),
    CreationErrorCopied,
    Imported,
    ImportConfirmed,
    ImportCanceled,
    ImportProjectPathChanged(String),
    NonexistantRestored,
    NonexistantRemoved,
    RemoveNonexistantFinished(Result<usize, String>),
}
