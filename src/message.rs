use std::path::PathBuf;

use crate::app_state::{Project, ProjectType};

#[derive(Debug, Clone)]
pub enum Message
{
    Open(Project),
    Selected(usize),
    Remove,
    ConfirmRemove,
    CancelRemove,
    ToggleRemoveProjectFolder(bool),
    Create,
    ConfirmCreate,
    CancelCreate,
    ChangeNewProjectName(String),
    ChangeNewProjectPath(PathBuf),
    ChangeNewProjectType(ProjectType),
    Import,
}
