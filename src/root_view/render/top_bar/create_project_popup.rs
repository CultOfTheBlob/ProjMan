use crate::{
    app_state::{AppState, GlobalAppState},
    log::Log,
    project::{Nonexistant, Project},
};
use gpui::*;
use gpui_component::{
    input::InputState,
    select::{SearchableVec, SelectState},
};
use rfd::FileDialog;
use smol::channel;
use std::{fs, path::PathBuf, sync::Arc};

mod popup;
mod render;

pub struct CreateProjectPopup {
    focus_handle: FocusHandle,

    scroll_handle: ScrollHandle,

    project_name_input_state: Entity<InputState>,
    project_repo_input_state: Entity<InputState>,
    project_template_select_state: Entity<SelectState<SearchableVec<String>>>,
    project_path_input_state: Entity<InputState>,

    console_logs: Vec<String>,
}

impl CreateProjectPopup {
    fn select_directory_button_pressed(
        this: &mut Self,
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = FileDialog::new()
            .set_title("Pick Project Directory")
            .pick_folder()
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
        let app_state = cx.global::<GlobalAppState>().0.clone();

        let name = view.project_name_input_state.read(cx).value().to_string();
        let repo = view.project_repo_input_state.read(cx).value().to_string();

        let path =
            PathBuf::from(view.project_path_input_state.read(cx).value().to_string());

        let template_name = view
            .project_template_select_state
            .read(cx)
            .selected_value()
            .cloned()
            .unwrap_or_default();

        let view_entity = cx.entity();

        let (sender, reciever) = channel::unbounded::<String>();

        let log_to_console = move |mut cx: AsyncApp| async move {
            while let Ok(line) = reciever.recv().await {
                let update = |view: &mut Self, cx: &mut Context<Self>| {
                    view.console_logs.push(line);

                    let last_index = view.console_logs.len().saturating_sub(1);
                    view.scroll_handle.scroll_to_item(last_index);

                    cx.notify();
                };

                let _ = cx.update_entity(&view_entity, update);
            }
        };

        cx.spawn(|_, cx: &mut AsyncApp| {
            let cx = cx.clone();

            log_to_console(cx)
        })
        .detach();

        let window_handle = window.window_handle();

        AppState::set_creating_project(cx, true);

        let create = move |mut cx: AsyncApp| async move {
            let project = cx
                .background_executor()
                .spawn(async move {
                    let on_log = move |line: String| {
                        let _ = sender.send_blocking(line);
                    };

                    match Project::<Nonexistant>::new()
                        .name(name)
                        .repo(repo)
                        .path(path.clone())
                        .template_name(template_name)
                        .create(&app_state, on_log)
                        .await
                    {
                        Ok(project) => Some(project),
                        Err(err) => {
                            if let Err(err) = fs::remove_dir_all(path) {
                                Log::Error.log(&err.to_string());
                            }

                            Log::Error.log(&err.to_string());

                            None
                        }
                    }
                })
                .await;

            let _ = cx.update_global::<GlobalAppState, ()>(|app_state, _| {
                let app_state = Arc::make_mut(&mut app_state.0);

                if let Some(project) = project
                    && let Err(err) = app_state.add_project(project)
                {
                    Log::Error.log(&err.to_string());
                }

                app_state.creating_project = false;
            });

            let _ = cx.update_window(
                window_handle,
                |_, window: &mut Window, cx: &mut App| {
                    AppState::set_modal_active(cx, false);
                    window.remove_window();
                },
            );
        };

        cx.spawn(|_, cx: &mut AsyncApp| {
            let cx = cx.clone();

            create(cx)
        })
        .detach();
    }
}
