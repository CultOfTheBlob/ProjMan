use crate::{
    app_state::GlobalAppState,
    config::Config,
    root_view::{
        RootView, popup,
        render::{self, sidebar::edit_project_popup::EditProjectPopup},
    },
};
use gpui::{prelude::FluentBuilder as _, *};

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let disabled =
        app_state.get_selected_project().is_none() || app_state.restoring_project;

    let listener = move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
        if disabled {
            return;
        }

        popup::create::<EditProjectPopup>(&root_view, cx);
    };

    render::text_button(
        "sidebar_edit_button",
        "Edit",
        Some(""),
        &theme,
        Some(disabled),
    )
    .when(disabled, |this: Stateful<Div>| {
        this.bg(theme.background)
            .border_color(theme.background)
            .text_color(theme.text_disabled)
    })
    .on_click(listener)
}
