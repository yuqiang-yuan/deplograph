pub static APP_ID: &str = "app.deplograph";
pub static APP_NAME: &str = "Deplograph";

/// 应用自己的主题名，对应 assets/themes/deplograph.json 里的两个主题。
pub static THEME_LIGHT_NAME: &str = "Deplograph Light";
pub static THEME_DARK_NAME: &str = "Deplograph Dark";

/// Linux 窗口的 app_id（Wayland 的 app_id / X11 的 WM_CLASS）。
///
/// 必须等于打包时生成的桌面文件名 —— cargo-packager 固定把它命名成
/// `<二进制名>.desktop`，所以这里只能是 `deplograph`，不能跟着 [`APP_ID`]
/// 走，否则窗口和桌面项关联不上，任务栏图标显示不出来。
pub static APP_DESKTOP_ID: &str = "deplograph";
