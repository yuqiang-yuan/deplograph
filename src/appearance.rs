use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::{App, px};

use crate::settings::AppSettings;

/// 切深浅色：改主题、落盘、重画。标题栏的切换按钮用它。
pub fn set_dark_mode(is_dark: bool, cx: &mut App) {
    change_theme(is_dark, cx);
    write(cx, |settings| settings.is_dark = Some(is_dark));
    cx.refresh_windows();
}

/// 改字号：改主题、落盘、重画。标题栏的字号菜单用它。
pub fn set_font_size(size: f32, cx: &mut App) {
    change_font_size(size, cx);
    write(cx, |settings| settings.font_size = Some(size));
    cx.refresh_windows();
}

/// 一次应用整份设置。设置弹窗点「保存」用它。
pub fn apply(settings: &AppSettings, cx: &mut App) {
    change_theme(settings.is_dark(), cx);
    change_font_size(settings.font_size(), cx);

    let global = cx.global_mut::<AppSettings>();
    global.is_dark = settings.is_dark;
    global.font_size = settings.font_size;
    global.save();

    cx.refresh_windows();
}

/// `ThemeMode` 只有 Light / Dark，没有「跟随系统」：要跟系统就把
/// `window.appearance()` 的结果传进来。
fn change_theme(is_dark: bool, cx: &mut App) {
    Theme::change(
        if is_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        },
        None,
        cx,
    );
}

/// 字号是 base theme 的一部分，改完要同步给所有窗口，文字才会重排。
fn change_font_size(size: f32, cx: &mut App) {
    Theme::global_mut(cx).font_size = px(size);
    Theme::sync_base(cx);
}

/// 写进全局设置并落盘。
fn write(cx: &mut App, update: impl FnOnce(&mut AppSettings)) {
    let settings = cx.global_mut::<AppSettings>();
    update(settings);
    settings.save();
}
