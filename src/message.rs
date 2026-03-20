use std::path::PathBuf;

use crate::{
    error::Error,
    project::{Project, project_type::ProjectType},
};

#[derive(Debug, Clone)]
pub enum Message
{
    Opened(Project),
    Selected(usize),
    Deselected,
    Updated,
    Removed,
    RemoveConfirmed,
    RemoveCanceled,
    RemoveProjectFolderToggled(bool),
    Created,
    CreateConfirmed,
    ProjectDirCreated(Result<String, Error>),
    ProjectRepoCloned(Result<String, Error>),
    ProjmanFileCreated(Result<String, Error>),
    DirStructureCreated(Result<String, Error>),
    ProjectFilesCreated(Result<String, Error>),
    BuildCommandExecuted(usize, Result<String, Error>),
    CommitedProjmanInit(Result<String, Error>),
    ProjectAddedToJson(Result<Vec<Project>, Error>),
    CreateFinished(Result<(), Error>),
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
    ImportProjectNameChanged(String),
    NonexistantRestored,
    NonexistantRemoved,
    RemoveNonexistantFinished(Result<usize, Error>),
    NotificationRemoved(usize),
    NotificationCopied(usize),
}
