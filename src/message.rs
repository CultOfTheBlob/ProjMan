use crate::app_state::Project;

#[derive(Debug, Clone)]
pub enum Message
{
    Selected(usize),
    Open(Project),
}
