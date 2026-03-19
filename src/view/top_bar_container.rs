use color_eyre::owo_colors::OwoColorize;
use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row},
};

use crate::{message::Message, state::app_state::AppState};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let mut top_bar_content = row![
        button("Create").style(button::secondary).on_press_maybe(
            if state.pending.is_none()
            {
                Some(Message::Created)
            }
            else
            {
                None
            }
        ),
        button("Import").style(button::secondary).on_press_maybe(
            if state.pending.is_none()
            {
                Some(Message::Imported)
            }
            else
            {
                None
            }
        )
    ]
    .spacing(12);

    if let Some(index) = state.selected_project
        && state.project_list[index].exists
    {
        match state.project_list[index].is_outdated()
        {
            Ok(outdated) =>
            {
                if outdated
                {
                    top_bar_content = top_bar_content.push(
                        button("Update").style(button::primary).on_press_maybe(
                            if state.pending.is_none()
                            {
                                Some(Message::Updated)
                            }
                            else
                            {
                                None
                            },
                        ),
                    );
                }
            }
            Err(err) => eprintln!("{}", err.get_message().red()),
        }

        top_bar_content =
            top_bar_content.push(button("Remove").style(button::warning).on_press_maybe(
                if state.pending.is_none()
                {
                    Some(Message::Removed)
                }
                else
                {
                    None
                },
            ));
    }

    let top_bar = container(top_bar_content)
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

    column![top_bar, content].into()
}
