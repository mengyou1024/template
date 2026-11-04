//! WebAssembly application entry point for {{ project-name }}.

use std::borrow::Cow;

use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::{Icon, TitleBar};
use gpui_kit::{AssetSource, SharedString, Window, div, prelude::*};
use rust_embed::Embed;

// 初始化 rust-i18n, 加载 locales/ 目录下的翻译文件.
// 回退语言为 zh-CN (当请求的 key 在目标语言中不存在时使用).
rust_i18n::i18n!("locales", fallback = "zh-CN");

/// 内嵌的自定义图标资源.
#[derive(Embed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
struct EmbeddedAssets;

/// 应用资源源: 内嵌图标 → 内嵌的 GPUI Kit 组件图标.
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

// 显式内嵌 GPUI Kit 组件用到的图标 (例如 RadioGroup 的选中标记),
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
        Ok(None)
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut assets = EmbeddedAssets::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect::<Vec<_>>();
        assets.extend(ComponentIcons.list(path)?);
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

/// 应用主视图.
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
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
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

// === WebAssembly 入口 ===

#[cfg(target_family = "wasm")]
use std::cell::RefCell;

#[cfg(target_family = "wasm")]
thread_local! {
    /// 持有应用句柄, 保证页面使用期间不被释放.
    static APPLICATION: RefCell<Option<gpui_kit::ApplicationHandle>> = const { RefCell::new(None) };
}

/// 宿主平台占位入口, 让 crate 也能在桌面端编译 (供 IDE / rust-analyzer 解析).
#[cfg(not(target_family = "wasm"))]
pub fn run() {}

/// 在浏览器中启动应用, 由 index.html 注入的 JS 调用.
#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    use std::rc::Rc;
    use std::sync::Arc;

    use gpui_kit::component::{Root, theme::Theme};
    use gpui_kit::web::{CanvasFontFallback, WebBackendPreference, WebPlatform};
    use gpui_kit::{WindowOptions};

    console_error_panic_hook::set_once();
    gpui_kit::platform::web_init();

    // 浏览器允许用 WebGPU/WebGL 绘制内嵌字体缺失的字形 (如中文/emoji).
    let platform = Rc::new(WebPlatform::new_with_backend_and_font_fallback(
        false,
        WebBackendPreference::Auto,
        CanvasFontFallback::EmojiAndCjk,
    ));
    let http_client = Arc::new(platform.fetch_http_client());
    let app = gpui_kit::Application::with_platform(platform)
        .with_http_client(http_client)
        .with_assets(Assets::new());

    let launch = move |cx: &mut gpui_kit::App| {
        gpui_kit::component::init(cx);

        // 浏览器没有系统字体库, 必须在开窗/首次文本测量前注册字体.
        // IBM Plex Sans 同时充当 `.SystemUIFont` 的映射字体.
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(
                include_bytes!("../fonts/IBMPlexSans-Regular.ttf").as_slice(),
            )])
            .expect("Failed to load bundled fonts");

        Theme::update(cx, |theme| {
            theme.font_family = "IBM Plex Sans".into();
            theme.mono_font_family = "IBM Plex Sans".into();
        });

        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Root::new(cx.new(|_| AppView::new()), window, cx))
        })
        .expect("Failed to open window");

        cx.activate(true);
    };

    APPLICATION.with(|application| {
        *application.borrow_mut() = Some(app.run_embedded(launch));
    });

    Ok(())
}
