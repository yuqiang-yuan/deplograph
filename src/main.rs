mod appearance;
mod assets;
mod globals;
mod i18n;
mod main_view;
mod settings;
mod settings_dialog;

// 应用自己的翻译：locales/ui.yml，缺 key 时回落到 en。
// `i18n!` 在这里生成 `_rust_i18n_backend()`，下面的 `extend!` 会用到它。
rust_i18n::i18n!("locales", fallback = "en");

#[cfg(target_os = "linux")]
use gpui_kit::WindowDecorations;
use gpui_kit::{
    App, AppContext as _, WindowBounds, WindowOptions,
    component::{Theme, ThemeMode, ThemeRegistry, TitleBar},
    px, size,
};

use crate::{
    assets::AppAssets,
    globals::{APP_DESKTOP_ID, APP_ID, APP_NAME, THEME_DARK_NAME, THEME_LIGHT_NAME},
    main_view::MainView,
    settings::AppSettings,
};

fn main() {
    let app = gpui_kit::application().with_assets(AppAssets);

    // macOS 上关掉窗口应用还活着（Dock 图标还在），点 Dock 图标时系统回调
    // applicationShouldHandleReopen:，GPUI 转成这个 on_reopen。它只在「一个窗口
    // 都没有」时触发，所以不会和已有窗口打架。
    app.on_reopen(open_main_window);

    app.run(|cx| {
        // 组件自己的文案走 gpui-kit 内置的 locales；extend! 让本应用 locales/ui.yml 里
        // `gpui_component:` 命名空间下的翻译与内置文案合并（同名 key 以本应用为准）。
        //
        // 必须早于 gpui_kit::init，且整个进程只能注册一次（重复注册会 panic）。
        use gpui_kit::component as gpui_component;
        rust_i18n::extend!(gpui_component);

        gpui_kit::init(cx);
        cx.set_app_identity(APP_ID, APP_NAME);

        let settings = AppSettings::load();

        // 语言、主题、字号都必须在开窗口之前定好，否则第一帧画的是默认值
        i18n::init_from_settings(&settings);

        load_app_theme(cx);

        // 主题和字号必须在开窗口之前设好，否则第一帧画的是默认主题
        Theme::change(
            if settings.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
            None,
            cx,
        );
        Theme::global_mut(cx).font_size = px(settings.font_size());

        cx.set_global(settings);

        cx.on_app_quit(|cx| {
            // 注意顺序：gpui 的 shutdown 是「先同步跑观察者的函数体 → 再清空窗口
            // → 才置 quitting」，返回的 future 是之后才被 await 的。所以窗口信息
            // 必须在这里读完，挪进 async 块就拿不到了。
            if let Some(handle) = cx.windows().first().copied()
                && let Ok((size, maximized)) = cx.update_window(handle, |_, window, _| {
                    (window.bounds().size, window.is_maximized())
                })
            {
                let settings = cx.global_mut::<AppSettings>();
                settings.window_maximized = Some(maximized);

                // 最大化时 bounds 就是屏幕大小，存进去会把「还原后的尺寸」冲掉，
                // 所以只在非最大化时记宽高
                if !maximized {
                    settings.window_width = Some(size.width.as_f32());
                    settings.window_height = Some(size.height.as_f32());
                }
            }

            cx.global::<AppSettings>().save();
            async {}
        })
        .detach();

        cx.spawn(async move |cx| {
            cx.update(open_main_window);
        })
        .detach();
    });
}

/// 注册应用自己的主题，并让浅色、深色两套都用上它。
fn load_app_theme(cx: &mut App) {
    let Ok(json) = assets::load_str("themes/deplograph.json") else {
        // 主题是编译进二进制的，读不到说明打包漏了：留着 gpui-kit 默认主题继续跑
        return;
    };
    ThemeRegistry::global_mut(cx)
        .load_themes_from_str(&json)
        .ok();

    let light = ThemeRegistry::global(cx)
        .themes()
        .get(THEME_LIGHT_NAME)
        .cloned();
    let dark = ThemeRegistry::global(cx)
        .themes()
        .get(THEME_DARK_NAME)
        .cloned();

    if let Some(light) = light {
        Theme::global_mut(cx).light_theme = light;
    }
    if let Some(dark) = dark {
        Theme::global_mut(cx).dark_theme = dark;
    }
}

/// 打开主窗口。
///
/// 关掉窗口只是把窗口移除，应用本身不退出；用户再点一次 Dock 图标时 GPUI 回调
/// [`Application::on_reopen`]，那里也走这个函数把窗口重新开出来 —— 所以窗口的
/// 创建逻辑必须能重复执行。
fn open_main_window(cx: &mut App) {
    // on_reopen 只在「一个窗口都没有」时才触发，这里再兜一层，免得以后加了别的
    // 入口意外开出第二个窗口。
    if !cx.windows().is_empty() {
        cx.activate(true);
        return;
    }

    let settings = cx.global::<AppSettings>().clone();

    let bounds = WindowBounds::centered(
        size(
            px(settings.window_width.unwrap_or(1200.0)),
            px(settings.window_height.unwrap_or(800.0)),
        ),
        cx,
    );

    let options = WindowOptions {
        window_bounds: if settings.window_maximized() {
            Some(WindowBounds::Maximized(bounds.get_bounds()))
        } else {
            Some(bounds)
        },
        // 窗口不该被拖得比工具栏还窄
        window_min_size: Some(size(px(720.0), px(480.0))),

        // 桌面环境靠这个字段把窗口和 deplograph.desktop 关联起来。
        // 注意 cx.set_app_identity() 在 Linux 上是被忽略的（gpui-pre-linux 只拿
        // 它的 name 去做系统通知），真正生效的是这个字段。
        app_id: Some(APP_DESKTOP_ID.to_string()),

        // 窗口自己画标题栏，所以 Linux 要客户端装饰；其他平台保留系统 chrome
        #[cfg(target_os = "linux")]
        window_decorations: Some(WindowDecorations::Client),
        // 标题栏的几何形状，以及配套的窗口拖动约定
        ..TitleBar::window_options()
    };

    // open_window 自己会把视图包进 base::Root
    gpui_kit::open_window(options, cx, |window, cx| {
        cx.new(|cx| MainView::new(window, cx))
    })
    .expect("failed to open the main window");

    cx.activate(true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::AssetSource as _;

    #[test]
    fn app_assets_are_embedded() {
        // 图标按 AppAssets 的路径解析，也就是 assets/ 目录
        let icon = AppAssets.load("icons/deplograph.svg").unwrap();
        assert!(
            icon.is_some(),
            "assets/icons/deplograph.svg is not embedded"
        );
    }

    #[test]
    fn app_theme_parses() {
        let json = assets::load_str("themes/deplograph.json").expect("theme asset embedded");
        let theme_set: gpui_kit::component::ThemeSet =
            serde_json::from_str(&json).expect("theme json matches the theme schema");

        let names = theme_set
            .themes
            .iter()
            .map(|theme| theme.name.to_string())
            .collect::<Vec<_>>();
        assert!(names.contains(&THEME_LIGHT_NAME.to_string()), "{names:?}");
        assert!(names.contains(&THEME_DARK_NAME.to_string()), "{names:?}");

        // 两套主题都得真的解析出颜色，否则加载成功但界面还是默认配色
        for theme in &theme_set.themes {
            assert!(
                theme.colors.background.is_some(),
                "{} has no background color",
                theme.name
            );
        }
    }
}
