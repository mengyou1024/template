#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Cow;

use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::{Icon, Root, TitleBar};
use gpui_kit::{AssetSource, SharedString, Size, WindowBounds, WindowOptions, div, prelude::*, px};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use single_application::{PrimaryInstance, SingleApplication};
use time::macros::format_description;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

// 初始化 rust-i18n,加载 locales/ 目录下的翻译文件
// 回退语言为 zh-CN(当请求的 key 在目标语言中不存在时使用)
rust_i18n::i18n!("locales", fallback = "zh-CN");

/// 初始化日志: debug 模式输出到 stdout, release 模式输出到滚动文件.
/// 返回的 guard 必须在 main 存活期间保持存活(确保非阻塞写入刷新到文件).
#[cfg(not(debug_assertions))]
fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    let log_dir = dirs::config_dir()
        .expect("无法获取配置目录")
        .join("{{ project-name }}")
        .join("logs");
    std::fs::create_dir_all(&log_dir).expect("无法创建日志目录");

    let file_appender = tracing_appender::rolling::daily(&log_dir, "{{ project-name }}.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .pretty()
        .init();

    // 自定义 panic hook: 将 panic 转发到 tracing 日志(文件/stdout), 同时保留默认 stderr 输出.
    // release 模式下 _guard 在 panic 展开时被 drop, WorkerGuard::drop 会 join 后台线程,
    // 确保 tracing::error! 的 panic 消息已刷新到日志文件.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tracing::error!("panic: {}", panic_info);
        default_hook(panic_info);
    }));

    guard
}

/// 初始化日志: debug 模式输出到 stdout.
#[cfg(debug_assertions)]
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .pretty()
        .init();
}

/// An asset source that loads assets from the `./assets` folder.
#[derive(Embed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        match Self::get(path) {
            Some(file) => Ok(Some(file.data)),
            None => gpui_kit::assets::Assets.load(path),
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut assets = Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect::<Vec<_>>();
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

struct AppView {
    locale: AppLocale,
}

impl Render for AppView {
    fn render(
        &mut self,
        _window: &mut gpui_kit::Window,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum AppMessage {
    RequestAttention,
}

const APP_INSTANCE_ID: &str = "{{ app_instance_id }}";

#[tokio::main(worker_threads = 2)]
async fn main() {
    // 初始化日志: debug → stdout, release → 滚动文件(dirs::config_dir()/{{ project-name }}/logs)
    #[cfg(not(debug_assertions))]
    let _guard = init_logging();
    #[cfg(debug_assertions)]
    init_logging();

    let primary_instance = match SingleApplication::<AppMessage>::new(APP_INSTANCE_ID) {
        SingleApplication::PrimaryInstance(primary_instance) => primary_instance,
        SingleApplication::SecondaryInstance(secondary_instance) => {
            let _ = tokio::spawn(async move {
                // 发送唤醒消息
                secondary_instance
                    .send_message(AppMessage::RequestAttention)
                    .await;
            })
            .await;

            return;
        }
    };

    rust_i18n::set_locale("zh-CN");

    gpui_kit::platform::application()
        .with_assets(Assets)
        .run(move |cx| {
            gpui_kit::component::init(cx);

            let window_bounds = Some(WindowBounds::centered(Size::new(px(640.0), px(480.0)), cx));

            cx.spawn(async move |cx| {
                cx.open_window(
                    WindowOptions {
                        window_bounds,
                        titlebar: Some(TitleBar::title_bar_options()),
                        // 禁用窗口非活动时的帧间隔
                        inactive_frame_interval: None,
                        ..Default::default()
                    },
                    |window, cx| {
                        cx.new(|cx| {
                            create_message_receive_task(primary_instance, cx);

                            Root::new(
                                cx.new(|_| AppView {
                                    locale: AppLocale::SimplifiedChinese,
                                }),
                                window,
                                cx,
                            )
                        })
                    },
                )
                .expect("Failed to open window");
            })
            .detach();
        });
}

fn create_message_receive_task<T: 'static>(
    mut primary_instance: PrimaryInstance<AppMessage>,
    cx: &mut Context<T>,
) {
    cx.spawn(async move |entity, cx| {
        loop {
            let mut message = None;

            cx.background_executor()
                .scoped(|scope| {
                    scope.spawn(async {
                        message = primary_instance.recv().await;
                    });
                })
                .await;

            if let Some(AppMessage::RequestAttention) = message {
                cx.with_window(entity.entity_id(), |window, _cx| {
                    window.activate_window();
                    window.request_attention();
                });
            }
        }
    })
    .detach();
}
