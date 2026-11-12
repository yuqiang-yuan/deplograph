mod main_view;

#[cfg(target_os = "linux")]
use gpui_kit::WindowDecorations;
use gpui_kit::component::TitleBar;
use gpui_kit::{AppContext as _, WindowBounds, WindowOptions, px, size};

use crate::main_view::MainView;

/// Reverse-DNS identity the platform layer uses for the app id and window class.
pub const APP_ID: &str = "app.deplograph";
pub const APP_NAME: &str = "deplograph";

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            // Initialises the enabled gpui-kit layers, so it runs before any
            // component-backed view is created.
            gpui_kit::init(cx);
            cx.set_app_identity(APP_ID, APP_NAME);

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::centered(size(px(1200.), px(800.)), cx)),
                window_min_size: Some(size(px(720.), px(480.))),
                // The shell draws its own title bar, so Linux wants client-side
                // decorations; other platforms keep their native chrome.
                #[cfg(target_os = "linux")]
                window_decorations: Some(WindowDecorations::Client),
                // Title bar geometry and the window-move contract that goes with it.
                ..TitleBar::window_options()
            };

            // `open_window` already wraps the view in `base::Root`.
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| MainView::new(window, cx))
            })
            .expect("failed to open the main window");
        });
}
