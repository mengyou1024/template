//! WebAssembly entry point for {{ project-name }}.

#[cfg(target_family = "wasm")]
use std::cell::RefCell;

#[cfg(target_family = "wasm")]
thread_local! {
    /// 持有应用句柄, 保证页面使用期间不被释放.
    static APPLICATION: RefCell<Option<gpui_kit::ApplicationHandle>> = const { RefCell::new(None) };
}

/// 桌面端占位入口, 使 web crate 也能在宿主平台上编译.
#[cfg(not(target_family = "wasm"))]
pub fn run() {}

/// 在浏览器中启动应用, 由 `www/src/main.js` 调用.
#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run() -> Result<(), wasm_bindgen::JsValue> {
    use std::borrow::Cow;
    use std::rc::Rc;
    use std::sync::Arc;

    use app_ui::{AppView, Assets};
    use gpui_kit::component::Root;
    use gpui_kit::component::theme::Theme;
    use gpui_kit::web::{CanvasFontFallback, WebBackendPreference, WebPlatform};
    use gpui_kit::{App, AppContext, Application, WindowOptions};

    console_error_panic_hook::set_once();
    gpui_kit::platform::web_init();

    // 浏览器允许用 WebGPU/WebGL 绘制内嵌字体缺失的字形(如中文/emoji).
    let platform = Rc::new(WebPlatform::new_with_backend_and_font_fallback(
        false,
        WebBackendPreference::Auto,
        CanvasFontFallback::EmojiAndCjk,
    ));
    let http_client = Arc::new(platform.fetch_http_client());
    let app = Application::with_platform(platform)
        .with_http_client(http_client)
        .with_assets(Assets::new());

    let launch = move |cx: &mut App| {
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
