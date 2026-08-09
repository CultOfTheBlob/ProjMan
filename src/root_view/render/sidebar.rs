use crate::{
    app_state::GlobalAppState, config::Config, project::valid_project::ValidProject,
    root_view::RootView,
};
use gpui::*;
use gpui_animation::{animation::TransitionExt as _, transition::general::EaseInOutQuad};
use std::time::Duration;

mod edit_project_button;
mod edit_project_popup;
mod open_project_button;
mod project_info;
mod remove_project_button;
mod remove_project_popup;

const SIDEBAR_WITDH: f32 = 640.0;

pub fn render(cx: &Context<RootView>, sidebar_open: bool) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let divider_line = div()
        .w_full()
        .h(px(2.0))
        .bg(theme.border)
        .rounded_full()
        .mb_4();

    let open_project_button = open_project_button::render(cx);

    let edit_project_button = edit_project_button::render(cx);

    let remove_project_button = remove_project_button::render(cx);

    let project = app_state.get_selected_project();

    let project_info = project.map_or_else(div, |project| match project {
        ValidProject::Existant(project) => project_info::render(cx, &project),
        ValidProject::Nonexistant(_) => div(),
    });

    div()
        .id("sidebar")
        .w(px(if sidebar_open { SIDEBAR_WITDH } else { 0.0 }))
        .h_full()
        .overflow_hidden()
        .with_transition("sidebar")
        .transition_when(
            !sidebar_open,
            Duration::from_millis(500),
            EaseInOutQuad,
            |style| style.w(px(0.0)),
        )
        .transition_when(
            sidebar_open,
            Duration::from_millis(500),
            EaseInOutQuad,
            |style| style.w(px(SIDEBAR_WITDH)),
        )
        .child(
            div()
                .w(px(SIDEBAR_WITDH))
                .h_full()
                .bg(theme.background_weak)
                .border_color(theme.border)
                .flex()
                .flex_col()
                .p_4()
                .text_color(theme.text_muted)
                .child(divider_line)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .w_full()
                        .child(open_project_button)
                        .child(edit_project_button)
                        .child(remove_project_button),
                )
                .child(div().h(px(40.0)))
                .child(project_info),
        )
}
