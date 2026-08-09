use crate::{
    config::Config,
    root_view::{
        RootView, popup,
        render::{self, top_bar::import_project_popup::ImportProjectPopup},
    },
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let root_view = cx.entity();

    render::text_button("top_bar_import_button", "Import", None, &theme, None).on_click(
        move |_, _, cx: &mut App| {
            popup::create::<ImportProjectPopup>(&root_view, cx);
        },
    )
}
