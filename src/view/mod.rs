mod project_list_container;
mod remove_popup;
mod top_bar_container;

use iced::{Element, widget::container};

use crate::{app_state::AppState, message::Message};

pub fn view(state: &AppState) -> Element<'_, Message>
{
    let content = container("").width(0).height(0).into();

    let content = top_bar_container::build(state, project_list_container::build(state, content));

    remove_popup::build(state, content)
}
