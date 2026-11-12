use std::{
    fs,
    path::{Path, PathBuf},
};

use gpui_kit::Global;
use serde::{Deserialize, Serialize};

use crate::globals::APP_ID;

/// 字号档位：设置页和标题栏共用同一份列表。
pub const FONT_SIZES: [(f32, &str); 3] = [(14.0, "Small"), (16.0, "Regular"), (18.0, "Large")];

const DEFAULT_FONT_SIZE: f32 = 16.0;

/// 应用设置，存到 `$XDG_CONFIG_HOME/<app id>/settings.json`。
///
/// `serde(default)`：老配置文件里缺字段（新版本加的）时按 Default 补，而不是
/// 整个反序列化失败 —— 那样会把用户已存的字号、窗口大小一起清掉。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    /// 深色主题。None 表示还没选过，按浅色开。
    pub is_dark: Option<bool>,
    pub font_size: Option<f32>,
    pub window_width: Option<f32>,
    pub window_height: Option<f32>,
    pub window_maximized: Option<bool>,
}

impl Global for AppSettings {}

impl AppSettings {
    pub fn path() -> Option<PathBuf> {
        Some(dirs::config_local_dir()?.join(APP_ID).join("settings.json"))
    }

    /// 保存设置。失败就忽略 —— 设置不值得为它打断用户的当前操作。
    pub fn save(&self) {
        if let Some(path) = Self::path() {
            self.save_to(&path);
        }
    }

    /// 读设置。读不到或解析失败就用默认值。
    pub fn load() -> Self {
        match Self::path() {
            Some(path) => Self::load_from(&path),
            None => Self::default(),
        }
    }

    fn save_to(&self, path: &Path) {
        if let Some(dir) = path.parent()
            && !dir.exists()
            && fs::create_dir_all(dir).is_err()
        {
            return;
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    fn load_from(path: &Path) -> Self {
        if path.exists()
            && let Ok(json) = fs::read_to_string(path)
            && let Ok(settings) = serde_json::from_str::<Self>(&json)
        {
            settings
        } else {
            Self::default()
        }
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark.unwrap_or_default()
    }

    pub fn font_size(&self) -> f32 {
        self.font_size.unwrap_or(DEFAULT_FONT_SIZE)
    }

    pub fn window_maximized(&self) -> bool {
        self.window_maximized.unwrap_or_default()
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            is_dark: Some(true),
            font_size: Some(DEFAULT_FONT_SIZE),
            window_width: Some(1200.0),
            window_height: Some(800.0),
            window_maximized: Some(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_round_trip() {
        let dir = std::env::temp_dir().join("deplograph-settings-test");
        let path = dir.join("settings.json");
        let _ = fs::remove_dir_all(&dir);

        let settings = AppSettings {
            is_dark: Some(false),
            font_size: Some(14.0),
            window_width: Some(1000.0),
            window_height: Some(700.0),
            window_maximized: Some(true),
        };
        settings.save_to(&path);

        let loaded = AppSettings::load_from(&path);
        assert_eq!(loaded.is_dark, Some(false));
        assert_eq!(loaded.font_size, Some(14.0));
        assert_eq!(loaded.window_width, Some(1000.0));
        assert_eq!(loaded.window_maximized, Some(true));

        // 老配置文件缺字段时按 Default 补，而不是整个反序列化失败
        fs::write(&path, "{\"is_dark\": false}").unwrap();
        let partial = AppSettings::load_from(&path);
        assert!(!partial.is_dark());
        assert_eq!(partial.font_size(), DEFAULT_FONT_SIZE);

        // 文件不存在、内容不是 JSON 都退到默认值
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(
            AppSettings::load_from(&path).is_dark,
            AppSettings::default().is_dark
        );
        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, "not json").unwrap();
        assert_eq!(
            AppSettings::load_from(&path).is_dark,
            AppSettings::default().is_dark
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
