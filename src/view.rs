use iced::{
    Element, Length,
    widget::{column, container, mouse_area, row, scrollable, text},
};

use crate::{app_state::AppState, message::Message};

pub fn view(state: &AppState) -> Element<'_, Message>
{
    let mut content = column![text("Projects:").size(32)].spacing(4);

    for (index, project) in state.project_list.iter().enumerate()
    {
        let is_selected = state.selected_project == Some(index);

        let project_content = row![
            text(&project.name),
            text(format!("{:?}", &project.project_type))
        ]
        .spacing(24);

        let row_container = container(project_content)
            .width(Length::Fill)
            .padding(10)
            .style(
                if is_selected
                {
                    container::bordered_box
                }
                else
                {
                    container::transparent
                },
            );

        let clickable = mouse_area(row_container)
            .on_press(Message::Selected(index))
            .on_double_click(Message::Open(project.clone()));

        content = content.push(clickable);
    }

    scrollable(content).into()
}
