# 模板列表

## Rust

- `rust/axum`
  - 功能: Axum Web 应用模板, Workspace 结构, 集成 Sea-ORM + validator + Docker + Dev Container.
- `rust/cli`
  - 功能: Rust CLI 应用模板, 基于 clap 库, 含子命令、参数解析与日志系统.
- `rust/dev-container`
  - 功能: Rust 容器化开发模板.
- `rust/ffi/cargo-cxx`
  - 功能: Rust + C++ FFI 互操作模板(基于 cxx).
- `rust/ffi/cmake-preset`
  - 功能: Rust + C++ FFI 互操作模板(基于 cmake-preset).
- `rust/gpui-kit-desktop`
  - 功能: GPUI 桌面应用模板, 基于 gpui-kit 组件库, Workspace 结构, i18n + 单实例 + 日志系统, 使用 cargo-generate 生成项目并自动为单实例应用生成唯一 UUID.
- `rust/gpui-kit-mobile`
  - 功能: GPUI Android 原生应用模板, 基于 gpui-kit + gpui-pre-mobile, NativeActivity + cargo-ndk 交叉编译, Gradle 工程打包 APK, 一键构建脚本 `android/build.bat`.
- `rust/gpui-kit-web`
  - 功能: GPUI WebAssembly 模板, 基于 gpui-kit Web 后端, Trunk 构建 (无需 Node.js), 单一 crate 包含视图/资源/i18n/字体嵌入, 根目录直接 `trunk serve` 启动开发服务器.
- `rust/proc-macro`
  - 功能: Rust 过程宏模板, Workspace 结构(facade crate + proc-macro crate), 基于 manyhow/darling/syn/quote/proc-macro2, 含 derive、attribute、function-like 三类宏示例、错误聚合输出与集成测试.

## C++

- `cpp/cmake/vcpkg-rust-ffi`
  - 功能: CMake + vcpkg 的 C++ 项目模板, 并集成 Rust 静态库.

## Github Action

- `github-action/docker/rust`
  - 功能: Rust 语言的 GitHub Action 模板.
