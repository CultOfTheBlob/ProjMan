use iced::Task;

use crate::{app_state::AppState, message::Message};

pub fn boot() -> (AppState, Task<Message>)
{
    (AppState::default(), Task::none())
}
