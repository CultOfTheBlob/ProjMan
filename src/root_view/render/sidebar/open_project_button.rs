use crate::{
    app_state::GlobalAppState,
    config::Config,
    log::Log,
    project::valid_project::ValidProject,
    root_view::{RootView, render},
};
use gpui::{prelude::FluentBuilder, *};

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let button =
        render::text_button("sidebar_open_button", "Open", Some(""), &theme, Some(true))
            .bg(theme.background)
            .border_color(theme.background)
            .text_color(theme.text_disabled);

    let Some(project) = app_state.get_selected_project() else {
        return button;
    };

    let disabled = matches!(project, ValidProject::Nonexistant(_));

    let button = render::text_button(
        "sidebar_open_button",
        "Open",
        Some(""),
        &theme,
        Some(disabled),
    );

    let listener = move |_: &ClickEvent, _: &mut Window, _: &mut App| {
        let ValidProject::Existant(ref project) = project else {
            return;
        };

        if let Err(err) = app_state.run_project(project) {
            Log::Error.log(&err.to_string());
        }
    };

    button
        .when(disabled, |this: Stateful<Div>| {
            this.bg(theme.background)
                .border_color(theme.background)
                .text_color(theme.text_disabled)
        })
        .on_click(listener)
}
