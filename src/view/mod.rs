mod create_popup;
mod import_popup;
mod notifications_container;
mod project_list_container;
mod remove_popup;
mod top_bar_container;

use iced::{Element, widget::container};

use crate::{message::Message, state::app_state::AppState};

pub fn view(state: &AppState) -> Element<'_, Message>
{
    let mut content = container("").width(0).height(0).into();

    content = top_bar_container::build(state, project_list_container::build(state, content));

    content = remove_popup::build(state, content);
    content = create_popup::build(state, content);
    content = import_popup::build(state, content);
    content = notifications_container::build(state, content);

    content
}
