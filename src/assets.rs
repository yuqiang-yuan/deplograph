use gpui_kit::{AssetSource, SharedString};
use rust_embed::Embed;

// 默认图标集之外的图标。
//
// gpui-kit 自带 1830 个图标的清单，但只有 `default-icons.txt` 里那 106 个会
// 编进二进制。用到清单外的图标（比如 `Waypoints`）就在这里列出来，名字取自
// gpui-kit-assets 的 `assets/icons/<kebab-case>.svg`。
//
// 注意这里不能用 `///`：宏展开出来的项拿不到 rustdoc。
gpui_kit::assets::icon_assets!(ExtraIcons, []);

/// 应用自己的资源：assets/ 目录下的图标和主题。
#[derive(Embed)]
#[folder = "assets"]
pub struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if let Some(file) = AppAssets::get(path) {
            return Ok(Some(file.data));
        }

        if let Some(data) = ExtraIcons.load(path)? {
            return Ok(Some(data));
        }

        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut files = AppAssets::iter()
            .filter(|f| f.starts_with(path))
            .map(SharedString::from)
            .collect::<Vec<_>>();
        files.extend(gpui_kit::assets::Assets.list(path)?);
        files.extend(ExtraIcons.list(path)?);
        Ok(files)
    }
}

/// 读一个编译进二进制的文本资源（主题 JSON 这类）。
pub fn load_str(path: &str) -> anyhow::Result<String> {
    AppAssets::get(path)
        .map(|file| String::from_utf8_lossy(&file.data).into_owned())
        .ok_or_else(|| anyhow::anyhow!("asset not found: {path}"))
}
