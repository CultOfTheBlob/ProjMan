use std::sync::Arc;

use crate::{
    app_state::{AppState, GlobalAppState},
    log::Log,
};
use gpui::*;
use gpui_component::input::InputState;

mod popup;
mod render;

pub struct EditProjectPopup {
    focus_handle: FocusHandle,

    project_name_input_state: Entity<InputState>,
    project_repo_input_state: Entity<InputState>,
}

impl EditProjectPopup {
    fn close_button_pressed(
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        AppState::set_modal_active(cx, false);
        window.remove_window();
    }

    fn confirm_button_pressed(
        view: &mut Self,
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.update_global::<GlobalAppState, ()>(
            |app_state: &mut GlobalAppState, cx: &mut Context<Self>| {
                let app_state = Arc::make_mut(&mut app_state.0);

                let Some(project_index) = app_state.selected_project_index else {
                    return;
                };

                let name = view.project_name_input_state.read(cx).value().to_string();
                let repo = view.project_repo_input_state.read(cx).value().to_string();

                if let Err(err) = app_state.edit_project(project_index, name, repo) {
                    Log::Error.log(&err.to_string());
                }
            },
        );

        AppState::set_modal_active(cx, false);
        window.remove_window();
    }
}
