use crate::{
    app_state::{AppState, GlobalAppState},
    config::Config,
    log::Log,
    root_view::RootView,
};
use gpui::*;
use gpui_component::{Root, Theme as ComponentTheme, ThemeConfig};
use gpui_component_assets::Assets;
use std::{rc::Rc, sync::Arc};

/**
# Panics
If the `App::open_window` method fails and GPUI cant create a window.
*/
pub fn run_app(config: Config) {
    Application::new().with_assets(Assets).run(move |cx| {
        gpui_component::init(cx);

        let theme = config.theme.theme.get_theme();
        ComponentTheme::global_mut(cx).apply_config(&Rc::new(ThemeConfig::from(theme)));

        let app_state = match AppState::new() {
            Ok(app_state) => app_state,
            Err(err) => {
                Log::Error.log(&err.to_string());

                return;
            }
        };

        cx.set_global(config);
        cx.set_global(GlobalAppState(Arc::new(app_state)));

        let options = {
            let window_bounds = Some(WindowBounds::Windowed(Bounds::maximized(None, cx)));

            let titlebar = Some(TitlebarOptions {
                title: Some(SharedString::from("ProjMan")),
                appears_transparent: false,
                ..Default::default()
            });

            WindowOptions {
                app_id: Some(String::from("projman")),
                titlebar,
                window_bounds,
                kind: WindowKind::Normal,
                ..Default::default()
            }
        };

        let build_root_view = |window: &mut Window, cx: &mut App| {
            window.on_window_should_close(cx, |_, cx: &mut App| {
                cx.quit();

                true
            });

            let view = cx.new(|cx: &mut Context<RootView>| RootView::new(window, cx));

            cx.new(|cx| Root::new(view, window, cx))
        };

        if let Err(err) = cx
            .open_window(options, build_root_view)
            .map_err(|err| crate::error::Error::CreateWindow(err.to_string()))
        {
            panic!("PANIC: {err}");
        }
    });
}
