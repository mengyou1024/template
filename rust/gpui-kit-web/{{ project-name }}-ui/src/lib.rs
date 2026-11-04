//! Shared views and assets used by both the desktop and the web application.

use std::borrow::Cow;

use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::{Icon, TitleBar};
use gpui_kit::{AssetSource, SharedString, Window, div, prelude::*};
use rust_embed::Embed;

// 初始化 rust-i18n,加载 locales/ 目录下的翻译文件
// 回退语言为 zh-CN(当请求的 key 在目标语言中不存在时使用)
rust_i18n::i18n!("locales", fallback = "zh-CN");

/// 内嵌的自定义资源(桌面与 Web 均可用).
#[derive(Embed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
struct EmbeddedAssets;

/// 应用资源源: 内嵌的自定义图标 → 内嵌的组件图标 → GPUI Kit 默认图标(仅桌面).
///
/// Web 端没有文件系统, 也不会联网下载图标: 未列出的图标返回空.
/// 需要新的组件图标时, 把名称加到下面的 `icon_assets!` 列表中.
pub struct Assets;

impl Assets {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

// 显式内嵌 GPUI Kit 组件用到的图标(例如 RadioGroup 的选中标记),
// 避免 Web 端去请求不存在的 `/assets/icons/*.svg` 并反复报错.
gpui_kit::assets::icon_assets!(ComponentIcons, [Check]);

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        if let Some(file) = EmbeddedAssets::get(path) {
            return Ok(Some(file.data));
        }

        if let Some(data) = ComponentIcons.load(path)? {
            return Ok(Some(data));
        }

        #[cfg(not(target_family = "wasm"))]
        return gpui_kit::assets::Assets.load(path);

        #[cfg(target_family = "wasm")]
        Ok(None)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut assets = EmbeddedAssets::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect::<Vec<_>>();

        assets.extend(ComponentIcons.list(path)?);

        #[cfg(not(target_family = "wasm"))]
        assets.extend(gpui_kit::assets::Assets.list(path)?);

        Ok(assets)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppLocale {
    SimplifiedChinese,
    English,
}

impl From<usize> for AppLocale {
    fn from(index: usize) -> Self {
        match index {
            0 => Self::SimplifiedChinese,
            1 => Self::English,
            _ => Self::SimplifiedChinese,
        }
    }
}

impl AppLocale {
    fn as_str(self) -> &'static str {
        match self {
            Self::SimplifiedChinese => "zh-CN",
            Self::English => "en",
        }
    }
}

/// 应用主视图, 桌面端与 Web 端共用.
pub struct AppView {
    locale: AppLocale,
}

impl AppView {
    pub fn new() -> Self {
        Self {
            locale: AppLocale::SimplifiedChinese,
        }
    }
}

impl Default for AppView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(TitleBar::new().child(Icon::empty().path("icons/logo.svg")))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p_4()
                    .gap_4()
                    .child(
                        div().flex().gap_2().child(
                            RadioGroup::vertical("language-radio")
                                .children(["简体中文", "English"])
                                .selected_index(Some(self.locale as usize))
                                .on_change(cx.listener(|view, idx, _, cx| {
                                    view.locale = AppLocale::from(*idx);
                                    rust_i18n::set_locale(view.locale.as_str());
                                    cx.notify();
                                })),
                        ),
                    )
                    .child(rust_i18n::t!("hello_world")),
            )
    }
}
