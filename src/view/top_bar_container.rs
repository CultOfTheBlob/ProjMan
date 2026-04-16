use crate::{message::Message, state::app_state::AppState};
use iced::{
    Background::Color,
    Border, Element, Length, Theme,
    widget::{button, column, container, row, space, text, text_input},
};

pub fn build<'a>(state: &AppState, content: Element<'a, Message>) -> Element<'a, Message>
{
    let create_widget = button("Create").style(button::secondary).on_press_maybe(
        if state.pending.is_none()
        {
            Some(Message::Created)
        }
        else
        {
            None
        },
    );

    let import_widget = button("Import").style(button::secondary).on_press_maybe(
        if state.pending.is_none()
        {
            Some(Message::Imported)
        }
        else
        {
            None
        },
    );

    let filter_widget = container(
        text_input("Filter...", &state.project_list_filter)
            .style(
                |theme: &Theme, status: text_input::Status| text_input::Style {
                    background: Color(theme.extended_palette().background.weak.color),
                    border: Border {
                        color: match status
                        {
                            text_input::Status::Hovered
                            | text_input::Status::Focused { is_hovered: _ } =>
                            {
                                theme.extended_palette().primary.weak.color
                            }
                            text_input::Status::Disabled | text_input::Status::Active =>
                            {
                                theme.extended_palette().secondary.strong.color
                            }
                        },
                        width: 1.0,
                        radius: 4.into(),
                    },
                    ..text_input::default(theme, status)
                },
            )
            .on_input_maybe(
                if state.pending.is_none()
                {
                    Some(Message::ProjectListFilterChanged)
                }
                else
                {
                    None
                },
            ),
    );

    let sidebar_expand_widget =
        button(text(if state.sidebar_expanded { "" } else { "" }).size(24))
            .style(button::subtle)
            .on_press_maybe(
                if state.pending.is_none()
                {
                    Some(Message::SideBarToggled)
                }
                else
                {
                    None
                },
            );

    let top_bar_content = row![
        create_widget,
        import_widget,
        filter_widget,
        space().width(Length::Fill),
        sidebar_expand_widget
    ]
    .spacing(12);

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
