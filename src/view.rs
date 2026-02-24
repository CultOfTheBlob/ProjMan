use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, mouse_area, row, scrollable, text},
};

use crate::{app_state::AppState, message::Message};

pub fn view(state: &AppState) -> Element<'_, Message>
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

    let project_list_content = scrollable(project_list_content);

    let mut top_bar_content = row![
        button("Create")
            .style(button::secondary)
            .on_press(Message::Create),
        button("Import")
            .style(button::secondary)
            .on_press(Message::Import)
    ]
    .spacing(12);

    if let Some(index) = state.selected_project
    {
        top_bar_content = top_bar_content.push(
            button("Remove")
                .style(button::primary)
                .on_press(Message::Remove(index)),
        );
    }

    let project_list_container = container(
        container(project_list_content)
            .height(Length::Fill)
            .padding(16)
            .style(|theme: &Theme| container::Style {
                background: Some(Color(theme.extended_palette().background.weakest.color)),
                border: Border {
                    color: theme.extended_palette().background.strongest.color,
                    width: 1.0,
                    radius: 4.0.into(),
                },

                ..Default::default()
            }),
    )
    .padding(12);

    let top_bar_container = container(top_bar_content)
        .width(Length::Fill)
        .padding(8)
        .style(|theme: &Theme| container::Style {
            background: Some(Color(theme.extended_palette().background.weak.color)),
            border: Border {
                color: theme.extended_palette().background.strongest.color,
                width: 1.0,
                radius: 4.0.into(),
            },

            ..Default::default()
        });

    column![top_bar_container, project_list_container]
        .spacing(4)
        .into()
}
