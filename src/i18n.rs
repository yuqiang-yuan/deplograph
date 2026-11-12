use gpui_kit::App;

use crate::settings::AppSettings;

/// 目前支持的语言：码 + 用该语言自己写的名字。
///
/// 语言名故意不走 ui.yml —— 选项一律以自己的语言显示（English / 简体中文 / 繁體中文），
/// 这样即使切到一个看不懂的语言，也认得出自己那一项。
///
/// 码必须和 `locales/ui.yml` 里的码逐字一致，两者是同一份清单的两半：弹窗只会把这里的码
/// 交给 `set_locale`，所以不需要在 ui.yml 里再写 `zh-Hant` 之类的兜底。rust-i18n 只做精确
/// 匹配，之后逐段砍掉最后一个「-」再匹配，没有任何归一化 —— 写成 `zh_CN` 这种下划线形式
/// 永远匹配不上，测试里挡了这种情况。
pub const SUPPORTED_LOCALES: [(&str, &str); 3] =
    [("en", "English"), ("zh", "简体中文"), ("zh-HK", "繁體中文")];

/// 启动时按设置里的语言初始化。必须在开窗口之前调用，否则第一帧画的是默认语言。
pub fn init_from_settings(settings: &AppSettings) {
    rust_i18n::set_locale(settings.language());
}

/// 切语言：改 rust-i18n 的当前语言 → 落盘 → 重画。
pub fn set_locale(code: &str, cx: &mut App) {
    rust_i18n::set_locale(code);

    let settings = cx.global_mut::<AppSettings>();
    settings.language = Some(code.to_string());
    settings.save();

    // locale 是 GPUI 跟踪之外的全局状态，改完不会自动重绘，得手动刷
    cx.refresh_windows();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::FONT_SIZES;

    /// 界面文案 + 启动时按设置选语言。同一个测试进程里只有这一处改 locale，
    /// 所以断言都写在同一个测试里，免得并行跑时互相串。
    #[test]
    fn locales_and_copy() {
        // 默认（没保存过语言）是英语
        assert_eq!(rust_i18n::t!("app.settings"), "Settings");
        // 漏翻时 t! 原样返回 key，界面上一眼能看出来
        assert_eq!(rust_i18n::t!("app.not_defined"), "app.not_defined");

        // 设置弹窗用到的 key；字号那份直接从 FONT_SIZES 取，免得两处清单对不上
        let mut keys = vec![
            "app.settings",
            "app.save",
            "app.close",
            "app.theme",
            "app.theme_description",
            "app.theme_light",
            "app.theme_dark",
            "app.font_size",
            "app.font_size_description",
            "app.language",
            "app.language_description",
        ];
        keys.extend(FONT_SIZES.iter().map(|(_, key)| *key));

        for (code, label) in SUPPORTED_LOCALES {
            assert!(
                !code.contains('_'),
                "{code}: rust-i18n 只按 '-' 切分，下划线形式的码匹配不上"
            );
            assert!(!label.is_empty(), "{code} 少了语言名");

            rust_i18n::set_locale(code);
            for key in &keys {
                // t! 支持运行时 key：查不到就原样返回 key，所以这里能挡住漏翻和写错的 key
                assert_ne!(
                    rust_i18n::t!(*key),
                    *key,
                    "{code} 在 locales/ui.yml 里缺 {key} 的译文"
                );
            }
        }

        // 启动时按设置里的语言初始化
        let mut settings = AppSettings {
            language: Some("zh-HK".to_string()),
            ..AppSettings::default()
        };
        init_from_settings(&settings);
        assert_eq!(rust_i18n::t!("app.settings"), "設定");

        // 没保存过语言 → 英语
        settings.language = None;
        init_from_settings(&settings);
        assert_eq!(rust_i18n::t!("app.settings"), "Settings");
    }
}
