use crate::app_state::Project;

#[derive(Debug, Clone)]
pub enum Message
{
    Open(Project),
    Selected(usize),
    Remove(usize),
    RemoveProjectFolder(bool),
    ConfirmRemove,
    CancelRemove,
    Create,
    Import,
}
