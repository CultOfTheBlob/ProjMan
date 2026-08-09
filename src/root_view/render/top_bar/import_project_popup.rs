use crate::{
    app_state::{AppState, GlobalAppState},
    log::Log,
    project::Project,
};
use gpui::*;
use gpui_component::input::InputState;
use rfd::FileDialog;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

mod popup;
mod render;

pub struct ImportProjectPopup {
    focus_handle: FocusHandle,

    project_path_input_state: Entity<InputState>,
}

impl ImportProjectPopup {
    fn select_directory_button_pressed(
        this: &mut Self,
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let extension = Path::new(Project::<()>::PROJECT_FILE_NAME)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if let Some(path) = FileDialog::new()
            .set_title("Pick ProjMan File")
            .add_filter("ProjMan", &[extension])
            .pick_file()
        {
            let display_path = path.to_string_lossy().into_owned();

            this.project_path_input_state.update(
                cx,
                |input: &mut InputState, cx: &mut Context<InputState>| {
                    input.set_value(display_path, window, cx);
                },
            );
        }
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

                let path = view.project_path_input_state.read(cx).value().to_string();

                if let Err(err) = app_state.import_project(&PathBuf::from(path)) {
                    Log::Error.log(&err.to_string());
                }
            },
        );

        AppState::set_modal_active(cx, false);
        window.remove_window();
    }

    fn close_button_pressed(
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        AppState::set_modal_active(cx, false);
        window.remove_window();
    }
}
