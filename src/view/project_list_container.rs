use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{column, container, mouse_area, row, scrollable, text},
};

use crate::{app_state::AppState, message::Message};

pub fn build<'a>(state: &'a AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let mut project_list_content = column![].spacing(4);

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
                    |theme: &Theme| container::Style {
                        background: Some(Color(theme.extended_palette().background.weaker.color)),
                        border: Border {
                            color: theme.extended_palette().background.strongest.color,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }
                else
                {
                    container::transparent
                },
            );

        let clickable = mouse_area(row_container)
            .on_press(Message::Selected(index))
            .on_double_click(Message::Open(project.clone()));

        project_list_content = project_list_content.push(clickable);
    }

    let project_list = container(
        container(scrollable(project_list_content))
            .height(Length::Fill)
            .padding(16)
            .style(|theme: &Theme| container::Style {
                background: Some(Color(theme.extended_palette().background.base.color)),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }),
    )
    .padding(12)
    .style(|theme: &Theme| container::Style {
        background: Some(Color(theme.extended_palette().background.weakest.color)),
        ..Default::default()
    });

    column![project_list, content].into()
}
