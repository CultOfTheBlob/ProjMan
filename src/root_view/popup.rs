use crate::{app_state::AppState, log::Log, prelude::*, root_view::RootView};
use gpui::*;
use gpui_component::Root;

pub trait Popup {
    const TITLE: &'static str;

    const ID: &'static str;

    const WIDTH_FRACTION: f32;

    const HEIGHT_FRACTION: f32;

    fn create(root_view: &Entity<RootView>, window: &mut Window, cx: &mut App) -> Self;
}

pub fn create<T: Render + Popup>(root_view: &Entity<RootView>, cx: &mut App) {
    let options = {
        let display_bounds = cx.displays().first().map_or_else(
            || Bounds::new(Point::default(), Size::new(px(1920.0), px(1080.0))),
            |d| d.bounds(),
        );

        let popup_width = display_bounds.size.width * T::WIDTH_FRACTION;
        let popup_height = display_bounds.size.height * T::HEIGHT_FRACTION;
        let popup_size = Size::new(popup_width, popup_height);

        let window_bounds = Some(WindowBounds::Windowed(Bounds::centered(
            None, popup_size, cx,
        )));

        let titlebar = Some(TitlebarOptions {
            title: Some(SharedString::from(T::TITLE)),
            appears_transparent: false,
            ..Default::default()
        });

        WindowOptions {
            app_id: Some(String::from(T::ID)),
            window_bounds,
            titlebar,
            kind: WindowKind::Floating,
            ..Default::default()
        }
    };

    if let Err(err) = cx.open_window(options, |window: &mut Window, cx: &mut App| {
        window.on_window_should_close(cx, |_, cx: &mut App| {
            AppState::set_modal_active(cx, false);
            cx.refresh_windows();

            true
        });

        let view = cx.new(|cx: &mut Context<T>| T::create(root_view, window, cx));

        cx.new(|cx| Root::new(view, window, cx))
    }) {
        Log::Error
            .log(&Error::OpenPopup(T::TITLE.to_string(), err.to_string()).to_string());
    }

    AppState::set_modal_active(cx, true);
    cx.refresh_windows();
}
