use std::sync::Arc;

use crate::{
    app_state::{AppState, GlobalAppState},
    log::Log,
};
use gpui::*;

pub struct RemoveProjectPopup {
    remove_folder_checked: bool,
}

mod popup;
mod render;

impl RemoveProjectPopup {
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
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);

            let Some(project_index) = app_state.selected_project_index else {
                return;
            };

            if let Err(err) =
                app_state.remove_project(project_index, view.remove_folder_checked)
            {
                Log::Error.log(&err.to_string());
            }
        });

        AppState::set_modal_active(cx, false);
        window.remove_window();
    }

    fn remove_folder_checkbox_checked(
        this: &mut Self,
        _click_event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        this.remove_folder_checked = !this.remove_folder_checked;

        cx.notify();
    }
}
