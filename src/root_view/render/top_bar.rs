use crate::{config::Config, root_view::RootView};
use gpui::*;
use gpui_component::input::InputState;

mod collapse_sidebar_button;
mod create_project_button;
mod create_project_popup;
mod import_project_button;
mod import_project_popup;
mod search_bar;

pub fn render(
    cx: &Context<RootView>,
    sidebar_open: bool,
    search_bar_state: &Entity<InputState>,
) -> Div {
    let theme = cx.global::<Config>().theme.theme.get_theme();

    let create_project_button = create_project_button::render(cx);

    let import_project_button = import_project_button::render(cx);

    let search_bar = search_bar::render(search_bar_state);

    let seperator = div().flex_1();

    let collapse_sidebar_button = collapse_sidebar_button::render(cx, sidebar_open);

    div()
        .w_full()
        .h(px(48.0))
        .bg(theme.background_weak)
        .px_4()
        .flex()
        .flex_row()
        .items_center()
        .pt_2()
        .gap_2()
        .child(create_project_button)
        .child(import_project_button)
        .child(search_bar)
        .child(seperator)
        .child(collapse_sidebar_button)
}
