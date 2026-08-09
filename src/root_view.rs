use gpui::*;
use gpui_component::input::InputState;

mod popup;
mod render;

#[derive(Debug)]
pub struct RootView {
    focus_handle: FocusHandle,

    search_bar_state: Entity<InputState>,

    sidebar_open: bool,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_bar_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("Filter...")
        });

        Self {
            focus_handle: cx.focus_handle(),

            search_bar_state,

            sidebar_open: true,
        }
    }
}
