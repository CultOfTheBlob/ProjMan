use crate::{config::Config, root_view::RootView};
use gpui::*;

pub fn render(cx: &Context<RootView>, sidebar_open: bool) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();

    let bg_color = if sidebar_open {
        theme.surface
    } else {
        theme.background_weak
    };
    let border_color = if sidebar_open {
        theme.border
    } else {
        theme.background_weak
    };

    let listener =
        cx.listener(|view: &mut RootView, _, _, cx: &mut Context<RootView>| {
            view.sidebar_open = !view.sidebar_open;

            cx.notify();
        });

    div()
        .id("top_bar_toggle_sidebar_button")
        .h(px(32.0))
        .bg(bg_color)
        .text_color(theme.text_muted)
        .flex()
        .items_center()
        .justify_center()
        .px_2()
        .rounded_md()
        .border_1()
        .border_color(border_color)
        .cursor_pointer()
        .child(if sidebar_open { "◧" } else { "□" })
        .hover(|style: StyleRefinement| style.bg(theme.surface_strong))
        .active(|style: StyleRefinement| style.bg(theme.background_weak))
        .on_click(listener)
}
