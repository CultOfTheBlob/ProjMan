use crate::app_state::Project;

#[derive(Debug, Clone)]
pub enum Message
{
    Open(Project),
    Selected(usize),
}
