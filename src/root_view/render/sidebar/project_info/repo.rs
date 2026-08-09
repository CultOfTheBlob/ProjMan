use crate::{log::Log, theme::Theme};
use gpui::*;

pub fn render(repo: String, theme: &Theme) -> Div {
    let repo_name = repo.clone();

    let listener = move |_event: &ClickEvent, _window: &mut Window, _cx: &mut App| {
        let repo = if repo.starts_with("git@") {
            let repo = &repo.strip_prefix("git@").unwrap_or(&repo);
            let (host, path) = repo.split_once(':').unwrap_or_default();
            let path = path.trim_end_matches(".git");

            &format!("https://{host}/{path}")
        } else {
            &repo
        };

        if let Err(err) = open::that(repo) {
            Log::Error.log(&err.to_string());
        }
    };

    div()
        .flex()
        .items_center()
        .gap_2()
        .p_1()
        .child(
            div()
                .text_color(theme.accent)
                .font_weight(FontWeight::MEDIUM)
                .child("Repo:"),
        )
        .child(
            div()
                .id("sidebar_project_info_project_repo_url")
                .text_color(theme.text)
                .cursor_pointer()
                .active(|style: StyleRefinement| style.text_color(theme.accent_alt))
                .on_click(listener)
                .child(repo_name),
        )
}
