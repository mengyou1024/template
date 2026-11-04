//! GPUI Kit Mobile — {{ project-name }}
//!
//! Android 原生应用入口. NativeActivity dlopen 本 .so 后,
//! `android-activity` crate 在专用 native 线程上调用 `android_main`,
//! 直到 Activity 销毁才返回.
//!
//! ## Android 构建
//!
//! ```text
//! rustup target add aarch64-linux-android
//! cargo install cargo-ndk
//! cargo ndk -t arm64-v8a --platform 31 build --release
//! ```

#[cfg(target_os = "android")]
extern crate gpui_mobile;

#[cfg(target_os = "android")]
use gpui::{prelude::*, App, Application, WindowOptions};

// ═══════════════════════════════════════════════════════════════════════════
// Android Entry Point
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    use gpui_mobile::android::jni;

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("{{crate_name}}"),
    );

    jni::install_panic_hook();
    log::info!("android_main: entered");

    let _platform = jni::init_platform(&app);
    let shared = match jni::shared_platform() {
        Some(s) => s,
        None => {
            log::error!("android_main: shared_platform() returned None — aborting");
            return;
        }
    };

    log::info!("android_main: creating GPUI Application");

    Application::with_platform(shared.into_rc()).run(|cx: &mut App| {
        log::info!("Application::run callback — opening window");
        open_main_window(cx);
    });

    log::info!("android_main: Application.run returned — activity will finish");
}

// ═══════════════════════════════════════════════════════════════════════════
// Main Window
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(target_os = "android")]
fn open_main_window(cx: &mut App) {
    gpui_kit::init(cx);
    gpui_kit::component::Theme::change(gpui_kit::component::ThemeMode::Dark, None, cx);

    match cx.open_window(
        WindowOptions {
            window_bounds: None,
            ..Default::default()
        },
        |window, cx| {
            let view = cx.new(|_| AppView);
            cx.new(|cx| gpui_kit::component::Root::new(view, window, cx))
        },
    ) {
        Ok(_) => log::info!("cx.open_window succeeded"),
        Err(e) => log::error!("cx.open_window failed: {e:#}"),
    }

    cx.activate(true);
}

// ═══════════════════════════════════════════════════════════════════════════
// AppView — 主视图 (最小化 Hello World)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(target_os = "android")]
pub struct AppView;

#[cfg(target_os = "android")]
impl gpui_kit::Render for AppView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        use gpui_kit::component::TitleBar;
        gpui_kit::div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                gpui_kit::div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .p_4()
                    .gap_4()
                    .child("Hello from GPUI Kit Mobile!"),
            )
    }
}
