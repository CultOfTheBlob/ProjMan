use std::sync::Arc;

use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::{Existant, Project},
    root_view::RootView,
    theme::Theme,
};
use gpui::*;

mod authors;
mod branches;
mod commits;
mod languages;
mod metadata;
mod repo;

fn key_value(label: &'static str, value: String, theme: &Theme) -> Div {
    div()
        .flex()
        .items_start()
        .gap_2()
        .child(
            div()
                .flex_none()
                .text_color(theme.accent)
                .font_weight(FontWeight::MEDIUM)
                .child(label),
        )
        .child(div().flex_1().min_w_0().text_color(theme.text).child(value))
}

fn section_box(title: &'static str, content: Div, theme: &Theme) -> Div {
    div()
        .relative()
        .mt_3()
        .p_4()
        .rounded_lg()
        .border_1()
        .bg(theme.background_weak)
        .border_color(theme.border)
        .child(
            div()
                .flex()
                .absolute()
                .top(px(-1.0))
                .left_0()
                .right_0()
                .justify_center()
                .child(
                    div()
                        .px_3()
                        .py_0p5()
                        .bg(theme.surface)
                        .border_b_1()
                        .border_x_1()
                        .border_color(theme.border)
                        .rounded_b_md()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .child(title),
                ),
        )
        .child(div().pt_4().child(content))
}

pub fn render(cx: &Context<RootView>, project: &Arc<Project<Existant>>) -> Div {
    let app_state = cx.global::<GlobalAppState>().0.clone();
    let theme = cx.global::<Config>().theme.theme.get_theme();

    let project_info = if let Ok(info) = project.info(&app_state) {
        let repo = repo::render(project.repo.clone(), &theme);

        let branches = branches::render(&info.branches, info.current_branch, &theme);

        let languages = languages::render(&info.language_percentage, &theme);

        let authors = authors::render(&info.authors, &theme);

        let commits = commits::render(&info.last_commit, info.commit_count, &theme);

        let metadata = metadata::render(&info, &project.license, &theme);

        let div = div()
            .flex()
            .flex_col()
            .w_full()
            .gap_2()
            .text_sm()
            .text_color(theme.text)
            .child(repo)
            .child(branches)
            .child(languages)
            .child(authors)
            .child(commits)
            .child(metadata);

        Some(div)
    } else {
        None
    }
    .unwrap_or_else(|| div());

    div()
        .flex()
        .flex_col()
        .w_full()
        .bg(theme.surface)
        .rounded_xl()
        .border_1()
        .border_color(theme.border)
        .overflow_hidden()
        .child(
            div()
                .w_full()
                .bg(theme.background_weak)
                .px_4()
                .py_2p5()
                .border_b_1()
                .border_color(theme.border)
                .text_color(theme.text)
                .font_weight(FontWeight::BOLD)
                .child("Project Information"),
        )
        .child(
            div()
                .p_4()
                .flex_1()
                .text_color(theme.text_muted)
                .child(project_info),
        )
}
